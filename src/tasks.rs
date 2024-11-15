use std::time::Duration;

use crate::daqmx::{self, ActiveEdge, Result, Task, TerminalConfig};

/// A task that runs aquisitions of a finite length, with a configurable number of channels, sample rate, and aquisition duration.
pub struct FiniteAquisitionTask<const N: usize> {
    task: Task,
    num_samples: usize,
}
impl<const N: usize> FiniteAquisitionTask<N> {
    /// Creates a new FiniteAquisitionTask.
    ///
    /// * `name`: The name of the task.
    /// * `channels`: A list of physical channels to aquire
    /// * `sample_rate`: The sample rate in samples per second
    /// * `sample_time`: The total time of a single aqusition run
    pub fn new(
        name: &str,
        channels: [&str; N],
        sample_rate: f64,
        sample_time: Duration,
    ) -> Result<Self> {
        let task = Task::new(name)?;
        for channel in channels {
            task.create_voltage_input_channel(channel, "", TerminalConfig::Default, -10., 10.)?;
        }
        let num_samples = (sample_rate * sample_time.as_secs_f64()) as usize;
        task.configure_sample_clock(
            "",
            sample_rate,
            ActiveEdge::Rising,
            daqmx::SampleMode::Finite,
            num_samples as u64,
        )?;
        Ok(Self { task, num_samples })
    }
    /// Run a single aquisition
    ///
    /// Returns a list of vectors containing the data for each channel specified on task creation
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

/// A task that runs a continuous, with a configurable number of channels and sample rate.
pub struct ContinuousAquisitionTask<const C: usize> {
    task: Task,
    buffer: Vec<f64>,
}
impl<const C: usize> ContinuousAquisitionTask<C> {
    /// Creates a new ContinuousAquisitionTask.
    ///
    /// * `name`: The name of the task.
    /// * `channels`: A list of physical channels to aquire
    /// * `sample_rate`: The sample rate in samples per second
    /// * `buffer_time`: The time before the buffer will run out of room for samples.  [read_samples](ContinuousAquisitionTask::read_samples) must be run at least this often to not lose data.
    pub fn new(
        name: &str,
        channels: [&str; C],
        sample_rate: f64,
        buffer_time: Duration,
    ) -> Result<Self> {
        let task = Task::new(name)?;
        for channel in channels {
            task.create_voltage_input_channel(channel, "", TerminalConfig::Default, -10., 10.)?;
        }
        let num_samples = (sample_rate * buffer_time.as_secs_f64()) as usize;
        task.configure_sample_clock(
            "",
            sample_rate,
            ActiveEdge::Rising,
            daqmx::SampleMode::Continuous,
            num_samples as u64,
        )?;
        let buffer = vec![0f64; C * num_samples];
        Ok(Self { task, buffer })
    }
    /// Starts the aquisition
    pub fn start(&self) -> Result<()> {
        self.task.start()
    }
    /// Stops the aquisition
    pub fn stop(&self) -> Result<()> {
        self.task.stop()
    }
    /// Reads all availible data from the device
    /// 
    /// Returns an array of slices of readings for each channel
    pub fn read_samples<'a>(&'a mut self) -> Result<[&'a [f64]; C]> {
        let n = self.task.read_analog(-1, -1., false, &mut self.buffer)? as usize;
        let datasets: Vec<_> = (0..C).map(|i| &self.buffer[(n * i)..(n * (i + 1))]).collect();
        Ok(datasets.try_into().unwrap())
    }
}

/// A task that generates a repeating waveform on the specified channel.
pub struct WaveGenTask {
    task: Task,
    waveform: Vec<f64>,
    waveform_period: Duration,
}
impl WaveGenTask {
    /// Creates a new WaveGenTask.
    ///
    /// * `name`: The name of the task.
    /// * `channel`: The physical channel to output the waveform on.
    pub fn new(name: &str, channel: &str) -> Result<Self> {
        let task = Task::new(name)?;
        task.create_voltage_output_channel(channel, "", -10., 10.)?;
        Ok(Self {
            task,
            waveform: vec![],
            waveform_period: Duration::from_secs(1),
        })
    }
    /// Sets the waveform that is generated on the specified channel.
    ///
    /// * `buffer`: The buffer data that will be written to the device and generated on the specified channel.
    /// * `waveform_period`: The period of the generated waveform.
    pub fn set_waveform(&mut self, buffer: &[f64], waveform_period: Duration) {
        self.waveform = buffer.to_vec();
        self.waveform_period = waveform_period;
    }
    /// Starts generating the waveform.
    pub fn start(&self) -> Result<()> {
        self.download_waveform(&self.waveform, self.waveform_period, true)
    }
    /// Stops generating the waveform.
    pub fn stop(&self) -> Result<()> {
        self.task.stop()?;
        self.task.write_analog_value(false, -1., 0.)?;
        self.task.write_analog_value(true, -1., 0.)?;
        self.task.stop()?;
        Ok(())
    }
    fn download_waveform(
        &self,
        buffer: &[f64],
        waveform_period: Duration,
        auto_start: bool,
    ) -> Result<()> {
        self.task.configure_sample_clock(
            "",
            buffer.len() as f64 / waveform_period.as_secs_f64(),
            ActiveEdge::Rising,
            daqmx::SampleMode::Continuous,
            buffer.len() as u64,
        )?;
        self.task
            .write_analog(buffer.len() as i32, auto_start, -1., false, buffer)?;
        Ok(())
    }
}
