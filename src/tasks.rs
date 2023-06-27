use crate::daqmx::{self, ActiveEdge, Result, Task, TerminalConfig};

pub struct FiniteAquisition<const N: usize> {
    task: Task,
    num_samples: usize,
}
impl<const N: usize> FiniteAquisition<N> {
    pub fn new(
        name: impl AsRef<str>,
        channels: [&str; N],
        sample_rate: f64,
        sample_time: f64,
    ) -> Result<Self> {
        let task = Task::new(name)?;
        for channel in channels {
            task.create_voltage_input_channel(channel, "", TerminalConfig::RSE, -10., 10.)?;
        }
        let num_samples = (sample_rate * sample_time) as usize;
        task.configure_sample_clock(
            "",
            sample_rate,
            ActiveEdge::Rising,
            daqmx::SampleMode::Finite,
            num_samples as u64,
        )?;
        Ok(Self { task, num_samples })
    }
    pub fn run(&self) -> Result<[Vec<f64>; N]> {
        self.task.start()?;
        let mut datasets = vec![];
        let mut buffer = vec![0f64; self.num_samples * N];
        self.task
            .read_analog(self.num_samples as i32, -1., false, &mut buffer)?;
        self.task.stop()?;
        for i in 0..N {
            let slice = &buffer[self.num_samples * i..self.num_samples * (i + 1)];
            datasets.push(slice.to_vec());
        }
        Ok(datasets.try_into().unwrap())
    }
}
