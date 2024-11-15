use std::ffi::CString;

use crate::error::{Error, ErrorCoded};

pub(crate) mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Task {
    handle: bindings::TaskHandle,
}
unsafe impl Send for Task {}
unsafe impl Sync for Task {}
impl Task {
    /// Creates a task.
    ///
    /// * `name`: Name assigned to the task.
    pub fn new(name: impl AsRef<str>) -> Result<Self> {
        let name = CString::new(name.as_ref()).unwrap();
        let mut handle = 0 as bindings::TaskHandle;
        unsafe { bindings::DAQmxCreateTask(name.as_ptr(), &mut handle) }.check_code()?;
        Ok(Self { handle })
    }
    /// Creates channel(s) to measure voltage
    ///
    ///  * `channel`: The names of the physical channels to use to create virtual channels. You can specify a list or range of physical channels.
    ///  * `name`: The name(s) to assign to the created virtual channel(s). If you do not specify a name, NI-DAQmx uses the physical channel name as the virtual channel name. If you specify your own names for nameToAssignToChannel, you must use the names when you refer to these channels in other NI-DAQmx functions. If you create multiple virtual channels with one call to this function, you can specify a list of names separated by commas. If you provide fewer names than the number of virtual channels you create, NI-DAQmx automatically assigns names to the virtual channels.
    ///  * `terminal_config`: The input terminal configuration for the channel.
    ///  * `min_val`: The minimum value, in units, that you expect to measure.
    ///  * `max_val`: The maximum value, in units, that you expect to measure.
    pub fn create_voltage_input_channel(
        &self,
        channel: impl AsRef<str>,
        name: impl AsRef<str>,
        terminal_config: TerminalConfig,
        min_val: f64,
        max_val: f64,
    ) -> Result<()> {
        let channel = CString::new(channel.as_ref()).unwrap();
        let name = CString::new(name.as_ref()).unwrap();
        unsafe {
            bindings::DAQmxCreateAIVoltageChan(
                self.handle,
                channel.as_ptr(),
                name.as_ptr(),
                terminal_config as i32,
                min_val,
                max_val,
                bindings::DAQmx_Val_Volts as i32,
                &0,
            )
        }
        .check_code()
    }
    /// Creates channel(s) to generate voltage.
    ///
    ///  * `channel`: The names of the physical channels to use to create virtual channels. You can specify a list or range of physical channels.
    ///  * `name`: The name(s) to assign to the created virtual channel(s). If you do not specify a name, NI-DAQmx uses the physical channel name as the virtual channel name. If you specify your own names for nameToAssignToChannel, you must use the names when you refer to these channels in other NI-DAQmx functions. If you create multiple virtual channels with one call to this function, you can specify a list of names separated by commas. If you provide fewer names than the number of virtual channels you create, NI-DAQmx automatically assigns names to the virtual channels.
    ///  * `min_val`: The minimum value, in units, that you expect to generate.
    ///  * `max_val`: The maximum value, in units, that you expect to generate.
    pub fn create_voltage_output_channel(
        &self,
        channel: impl AsRef<str>,
        name: impl AsRef<str>,
        min_val: f64,
        max_val: f64,
    ) -> Result<()> {
        let channel = CString::new(channel.as_ref()).unwrap();
        let name = CString::new(name.as_ref()).unwrap();
        unsafe {
            bindings::DAQmxCreateAOVoltageChan(
                self.handle,
                channel.as_ptr(),
                name.as_ptr(),
                min_val,
                max_val,
                bindings::DAQmx_Val_Volts as i32,
                &0,
            )
        }
        .check_code()
    }
    /// Sets the source of the Sample Clock, the rate of the Sample Clock, and the number of samples to acquire or generate.
    ///
    /// * `source`: The source terminal of the Sample Clock. To use the internal clock of the device, use NULL or use OnboardClock.
    /// * `rate`: The sampling rate in samples per second per channel. If you use an external source for the Sample Clock, set this value to the maximum expected rate of that clock.
    /// * `active_edge`: Specifies on which edge of the clock to acquire or generate samples.
    /// * `sample_mode`: Specifies whether the task acquires or generates samples continuously or if it acquires or generates a finite number of samples.
    /// * `samples_per_channel`: The number of samples to acquire or generate for each channel in the task if sampleMode is Finite. If sampleMode is Continuous, NI-DAQmx uses this value to determine the buffer size.
    pub fn configure_sample_clock(
        &self,
        source: impl AsRef<str>,
        rate: f64,
        active_edge: ActiveEdge,
        sample_mode: SampleMode,
        samples_per_channel: u64,
    ) -> Result<()> {
        let source = CString::new(source.as_ref()).unwrap();
        unsafe {
            bindings::DAQmxCfgSampClkTiming(
                self.handle,
                source.as_ptr(),
                rate,
                active_edge as i32,
                sample_mode as i32,
                samples_per_channel,
            )
        }
        .check_code()
    }
    pub fn start(&self) -> Result<()> {
        unsafe { bindings::DAQmxStartTask(self.handle) }.check_code()
    }
    pub fn stop(&self) -> Result<()> {
        unsafe { bindings::DAQmxStopTask(self.handle) }.check_code()
    }
    /// Reads multiple floating-point samples from a task that contains one or more analog input channels.
    ///
    /// * `samples_per_channel`: The number of samples, per channel, to read. The default value of -1 reads all available samples. If `buffer` does not contain enough space, this function returns as many samples as fit in `buffer`. NI-DAQmx determines how many samples to read based on whether the task acquires samples continuously or acquires a finite number of samples. If the task acquires samples continuously and you set this parameter to -1, this function reads all the samples currently available in the buffer. If the task acquires a finite number of samples and you set this parameter to -1, the function waits for the task to acquire all requested samples, then reads those samples. If you set the Read All Available Samples property to TRUE, the function reads the samples currently available in the buffer and does not wait for the task to acquire all requested samples.
    /// * `timeout`: The amount of time, in seconds, to wait for the function to read the sample(s). To specify an infinite wait, pass -1. This function returns an error if the timeout elapses. A value of 0 indicates to try once to read the requested samples. If all the requested samples are read, the function is successful. Otherwise, the function returns a timeout error and returns the samples that were actually read.
    /// * `interleave`: Specifies whether or not the samples are interleaved.
    /// * `buffer`: The buffer to read samples into, organized according to `interleave`.
    ///
    /// This function returns the actual number of samples read from each channel.
    pub fn read_analog(
        &self,
        samples_per_channel: i32,
        timeout: f64,
        interleave: bool,
        buffer: &mut [f64],
    ) -> Result<i32> {
        let mut num_read = 0i32;
        unsafe {
            bindings::DAQmxReadAnalogF64(
                self.handle,
                samples_per_channel,
                timeout,
                interleave as u32,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                &mut num_read,
                std::ptr::null_mut(),
            )
        }
        .check_code()?;
        Ok(num_read)
    }
    /// Writes multiple floating-point samples to a task that contains one or more analog output channels.
    ///
    /// * `samples_per_channel`: The number of samples, per channel, to write. You must pass in a value of 0 or more in order for the sample to write. If you pass a negative number, this function returns an error.
    /// * `auto_start`: Specifies whether or not this function automatically starts the task if you do not start it.
    /// * `timeout`: The amount of time, in seconds, to wait for this function to write all the samples. To specify an infinite wait, pass -1. This function returns an error if the timeout elapses. A value of 0 indicates to try once to write the submitted samples. If this function successfully writes all submitted samples, it does not return an error. Otherwise, the function returns a timeout error and returns the number of samples actually written.
    /// * `interleave`: Specifies whether or not the samples are interleaved.
    /// * `buffer`: The buffer of 64-bit samples to write to the task.
    ///
    /// This function returns the actual number of samples per channel successfully written to the buffer.
    pub fn write_analog(
        &self,
        samples_per_channel: i32,
        auto_start: bool,
        timeout: f64,
        interleave: bool,
        buffer: &[f64],
    ) -> Result<i32> {
        let mut samples_written = 0;
        unsafe {
            bindings::DAQmxWriteAnalogF64(
                self.handle,
                samples_per_channel,
                auto_start as u32,
                timeout,
                interleave as u32,
                buffer.as_ptr(),
                &mut samples_written,
                std::ptr::null_mut(),
            )
        }
        .check_code()?;
        Ok(samples_written)
    }
    /// Writes a floating-point value to a task that contains one or more analog output channels.
    ///
    /// * `auto_start`: Specifies whether or not this function automatically starts the task if you do not start it.
    /// * `timeout`: The amount of time, in seconds, to wait for this function to write all the samples. To specify an infinite wait, pass -1. This function returns an error if the timeout elapses. A value of 0 indicates to try once to write the submitted samples. If this function successfully writes all submitted samples, it does not return an error. Otherwise, the function returns a timeout error and returns the number of samples actually written.
    /// * `value`: A 64-bit sample to write to the task.
    pub fn write_analog_value(&self, auto_start: bool, timeout: f64, value: f64) -> Result<()> {
        unsafe {
            bindings::DAQmxWriteAnalogScalarF64(
                self.handle,
                auto_start as u32,
                timeout,
                value,
                std::ptr::null_mut(),
            )
        }
        .check_code()
    }
}
impl Drop for Task {
    fn drop(&mut self) {
        unsafe { bindings::DAQmxClearTask(self.handle) };
    }
}

#[repr(i32)]
pub enum TerminalConfig {
    /// At run time, NI-DAQmx chooses the default terminal configuration for the channel.
    Default = bindings::DAQmx_Val_Cfg_Default,
    /// Referenced single-ended mode
    RSE = bindings::DAQmx_Val_RSE as i32,
    /// Non-referenced single-ended mode
    NRSE = bindings::DAQmx_Val_NRSE as i32,
    /// Differential mode
    Diff = bindings::DAQmx_Val_Diff as i32,
    /// Pseudodifferential mode
    PseudoDiff = bindings::DAQmx_Val_PseudoDiff as i32,
}

#[repr(i32)]
pub enum ActiveEdge {
    Rising = bindings::DAQmx_Val_Rising as i32,
    Falling = bindings::DAQmx_Val_Falling as i32,
}
#[repr(i32)]
pub enum SampleMode {
    Finite = bindings::DAQmx_Val_FiniteSamps as i32,
    Continuous = bindings::DAQmx_Val_ContSamps as i32,
    HWTimedSinglePoint = bindings::DAQmx_Val_HWTimedSinglePoint as i32,
}

pub type Result<T> = std::result::Result<T, Error>;
