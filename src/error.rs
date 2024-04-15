pub trait ErrorValue {
    fn check(self) -> Result<(), Error>;
}
impl ErrorValue for i32 {
    fn check(self) -> Result<(), Error> {
        match self.cmp(&0) {
            std::cmp::Ordering::Less => return Err(Error::try_from(self).unwrap()),
            std::cmp::Ordering::Greater => {
                tracing::warn!("{:?}", Warning::try_from(self).unwrap())
            }
            _ => {}
        }
        Ok(())
    }
}

#[repr(i32)]
#[derive(num_enum::TryFromPrimitive, Debug, thiserror::Error)]
pub enum Error {
    #[error("RemoteSense")]
    RemoteSense = -209888,
    #[error("OverTemperatureProtectionActivated")]
    OverTemperatureProtectionActivated = -209887,
    #[error("MultiTaskCfgSampRateNotSupportedWithPropSet")]
    MultiTaskCfgSampRateNotSupportedWithPropSet = -209886,
    #[error("MultiTaskCfgSampRateConflictingProp")]
    MultiTaskCfgSampRateConflictingProp = -209885,
    #[error("NoCommonSampRateFoundNoRepeatSamps")]
    NoCommonSampRateFoundNoRepeatSamps = -209884,
    #[error("NoCommonSampRateFound")]
    NoCommonSampRateFound = -209883,
    #[error("MultiTaskCfgDoesNotSupportMultiDevTask")]
    MultiTaskCfgDoesNotSupportMultiDevTask = -209882,
    #[error("MultiTaskSampRateCfgNotSupported")]
    MultiTaskSampRateCfgNotSupported = -209881,
    #[error("DebugSessionNotAllowedTimingSourceRegistered")]
    DebugSessionNotAllowedTimingSourceRegistered = -209880,
    #[error("DebugSessionNotAllowedWhenLogging")]
    DebugSessionNotAllowedWhenLogging = -209879,
    #[error("DebugSessionNotAllowedEventRegistered")]
    DebugSessionNotAllowedEventRegistered = -209878,
    #[error("InvalidTargetTaskForDebugSession")]
    InvalidTargetTaskForDebugSession = -209877,
    #[error("FunctionNotSupportedForDevice")]
    FunctionNotSupportedForDevice = -209876,
    #[error("MultipleTargetTasksFoundForDebugSession")]
    MultipleTargetTasksFoundForDebugSession = -209875,
    #[error("TargetTaskNotFoundForDebugSession")]
    TargetTaskNotFoundForDebugSession = -209874,
    #[error("OperationNotSupportedInDebugSession")]
    OperationNotSupportedInDebugSession = -209873,
    #[error("OperationNotPermittedInMonitorModeForDebugSession")]
    OperationNotPermittedInMonitorModeForDebugSession = -209872,
    #[error("GetActiveDevPrptyFailedDueToDifftVals")]
    GetActiveDevPrptyFailedDueToDifftVals = -209871,
    #[error("TaskAlreadyRegisteredATimingSource")]
    TaskAlreadyRegisteredATimingSource = -209870,
    #[error("FilterNotSupportedOnHWRev")]
    FilterNotSupportedOnHWRev = -209869,
    #[error("SensorPowerSupplyVoltageLevel")]
    SensorPowerSupplyVoltageLevel = -209868,
    #[error("SensorPowerSupply")]
    SensorPowerSupply = -209867,
    #[error("InvalidScanlist")]
    InvalidScanlist = -209866,
    #[error("TimeResourceCannotBeRouted")]
    TimeResourceCannotBeRouted = -209865,
    #[error("InvalidResetDelayRequested")]
    InvalidResetDelayRequested = -209864,
    #[error("ExceededTotalTimetriggersAvailable")]
    ExceededTotalTimetriggersAvailable = -209863,
    #[error("ExceededTotalTimestampsAvailable")]
    ExceededTotalTimestampsAvailable = -209862,
    #[error("NoSynchronizationProtocolRunning")]
    NoSynchronizationProtocolRunning = -209861,
    #[error("ConflictingCoherencyRequirements")]
    ConflictingCoherencyRequirements = -209860,
    #[error("NoSharedTimescale")]
    NoSharedTimescale = -209859,
    #[error("InvalidFieldDAQBankName")]
    InvalidFieldDAQBankName = -209858,
    #[error("DeviceDoesNotSupportHWTSP")]
    DeviceDoesNotSupportHWTSP = -209857,
    #[error("BankTypeDoesNotMatchBankTypeInDestination")]
    BankTypeDoesNotMatchBankTypeInDestination = -209856,
    #[error("InvalidFieldDAQBankNumberSpecd")]
    InvalidFieldDAQBankNumberSpecd = -209855,
    #[error("UnsupportedSimulatedBankForSimulatedFieldDAQ")]
    UnsupportedSimulatedBankForSimulatedFieldDAQ = -209854,
    #[error("FieldDAQBankSimMustMatchFieldDAQSim")]
    FieldDAQBankSimMustMatchFieldDAQSim = -209853,
    #[error("DevNoLongerSupportedWithinDAQmxAPI")]
    DevNoLongerSupportedWithinDAQmxAPI = -209852,
    #[error("TimingEngineDoesNotSupportOnBoardMemory")]
    TimingEngineDoesNotSupportOnBoardMemory = -209851,
    #[error("DuplicateTaskCrossProject")]
    DuplicateTaskCrossProject = -209850,
    #[error("TimeStartTriggerBeforeArmStartTrigger")]
    TimeStartTriggerBeforeArmStartTrigger = -209849,
    #[error("TimeTriggerCannotBeSet")]
    TimeTriggerCannotBeSet = -209848,
    #[error("InvalidTriggerWindowValue")]
    InvalidTriggerWindowValue = -209847,
    #[error("CannotQueryPropertyBeforeOrDuringAcquisition")]
    CannotQueryPropertyBeforeOrDuringAcquisition = -209846,
    #[error("SampleClockTimebaseNotSupported")]
    SampleClockTimebaseNotSupported = -209845,
    #[error("TimestampNotYetReceived")]
    TimestampNotYetReceived = -209844,
    #[error("TimeTriggerNotSupported")]
    TimeTriggerNotSupported = -209843,
    #[error("TimestampNotEnabled")]
    TimestampNotEnabled = -209842,
    #[error("TimeTriggersInconsistent")]
    TimeTriggersInconsistent = -209841,
    #[error("TriggerConfiguredIsInThePast")]
    TriggerConfiguredIsInThePast = -209840,
    #[error("TriggerConfiguredIsTooFarFromCurrentTime")]
    TriggerConfiguredIsTooFarFromCurrentTime = -209839,
    #[error("SynchronizationLockLost")]
    SynchronizationLockLost = -209838,
    #[error("InconsistentTimescales")]
    InconsistentTimescales = -209837,
    #[error("CannotSynchronizeDevices")]
    CannotSynchronizeDevices = -209836,
    #[error("AssociatedChansHaveAttributeConflictWithMultipleMaxMinRanges")]
    AssociatedChansHaveAttributeConflictWithMultipleMaxMinRanges = -209835,
    #[error("SampleRateNumChansOrAttributeValues")]
    SampleRateNumChansOrAttributeValues = -209834,
    #[error("WaitForValidTimestampNotSupported")]
    WaitForValidTimestampNotSupported = -209833,
    #[error("TrigWinTimeoutExpired")]
    TrigWinTimeoutExpired = -209832,
    #[error("InvalidTriggerCfgForDevice")]
    InvalidTriggerCfgForDevice = -209831,
    #[error("InvalidDataTransferMechanismForDevice")]
    InvalidDataTransferMechanismForDevice = -209830,
    #[error("InputFIFOOverflow3")]
    InputFIFOOverflow3 = -209829,
    #[error("TooManyDevicesForAnalogMultiEdgeTrigCDAQ")]
    TooManyDevicesForAnalogMultiEdgeTrigCDAQ = -209828,
    #[error("TooManyTriggersTypesSpecifiedInTask")]
    TooManyTriggersTypesSpecifiedInTask = -209827,
    #[error("MismatchedMultiTriggerConfigValues")]
    MismatchedMultiTriggerConfigValues = -209826,
    #[error("InconsistentAODACRangeAcrossTasks")]
    InconsistentAODACRangeAcrossTasks = -209825,
    #[error("InconsistentDTToWrite")]
    InconsistentDTToWrite = -209824,
    #[error("FunctionObsolete")]
    FunctionObsolete = -209823,
    #[error("NegativeDurationNotSupported")]
    NegativeDurationNotSupported = -209822,
    #[error("DurationTooSmall")]
    DurationTooSmall = -209821,
    #[error("DurationTooLong")]
    DurationTooLong = -209820,
    #[error("DurationBasedNotSupportedForSpecifiedTimingMode")]
    DurationBasedNotSupportedForSpecifiedTimingMode = -209819,
    #[error("InvalidLEDState")]
    InvalidLEDState = -209818,
    #[error("WatchdogStatesNotUniform")]
    WatchdogStatesNotUniform = -209817,
    #[error("SelfTestFailedPowerSupplyOutOfTolerance")]
    SelfTestFailedPowerSupplyOutOfTolerance = -209816,
    #[error("HWTSPMultiSampleWrite")]
    HWTSPMultiSampleWrite = -209815,
    #[error("OnboardRegenExceedsChannelLimit")]
    OnboardRegenExceedsChannelLimit = -209814,
    #[error("WatchdogChannelExpirationStateNotSpecified")]
    WatchdogChannelExpirationStateNotSpecified = -209813,
    #[error("InvalidShuntSourceForCalibration")]
    InvalidShuntSourceForCalibration = -209812,
    #[error("InvalidShuntSelectForCalibration")]
    InvalidShuntSelectForCalibration = -209811,
    #[error("InvalidShuntCalibrationConfiguration")]
    InvalidShuntCalibrationConfiguration = -209810,
    #[error("BufferedOperationsNotSupportedOnChannelStandalone")]
    BufferedOperationsNotSupportedOnChannelStandalone = -209809,
    #[error("FeatureNotAvailableOnAccessory")]
    FeatureNotAvailableOnAccessory = -209808,
    #[error("InconsistentThreshVoltageAcrossTerminals")]
    InconsistentThreshVoltageAcrossTerminals = -209807,
    #[error("DAQmxIsNotInstalledOnTarget")]
    DAQmxIsNotInstalledOnTarget = -209806,
    #[error("COCannotKeepUpInHWTimedSinglePoint")]
    COCannotKeepUpInHWTimedSinglePoint = -209805,
    #[error("WaitForNextSampClkDetected3OrMoreSampClks")]
    WaitForNextSampClkDetected3OrMoreSampClks = -209803,
    #[error("WaitForNextSampClkDetectedMissedSampClk")]
    WaitForNextSampClkDetectedMissedSampClk = -209802,
    #[error("WriteNotCompleteBeforeSampClk")]
    WriteNotCompleteBeforeSampClk = -209801,
    #[error("ReadNotCompleteBeforeSampClk")]
    ReadNotCompleteBeforeSampClk = -209800,
    #[error("InconsistentDigitalFilteringAcrossTerminals")]
    InconsistentDigitalFilteringAcrossTerminals = -201510,
    #[error("InconsistentPullUpCfgAcrossTerminals")]
    InconsistentPullUpCfgAcrossTerminals = -201509,
    #[error("InconsistentTermCfgAcrossTerminals")]
    InconsistentTermCfgAcrossTerminals = -201508,
    #[error("VCXODCMBecameUnlocked")]
    VCXODCMBecameUnlocked = -201507,
    #[error("PLLDACUpdateFailed")]
    PLLDACUpdateFailed = -201506,
    #[error("NoCabledDevice")]
    NoCabledDevice = -201505,
    #[error("LostRefClk")]
    LostRefClk = -201504,
    #[error("CantUseAITimingEngineWithCounters")]
    CantUseAITimingEngineWithCounters = -201503,
    #[error("DACOffsetValNotSet")]
    DACOffsetValNotSet = -201502,
    #[error("CalAdjustRefValOutOfRange")]
    CalAdjustRefValOutOfRange = -201501,
    #[error("ChansForCalAdjustMustPerformSetContext")]
    ChansForCalAdjustMustPerformSetContext = -201500,
    #[error("GetCalDataInvalidForCalMode")]
    GetCalDataInvalidForCalMode = -201499,
    #[error("NoIEPEWithACNotAllowed")]
    NoIEPEWithACNotAllowed = -201498,
    #[error("SetupCalNeededBeforeGetCalDataPoints")]
    SetupCalNeededBeforeGetCalDataPoints = -201497,
    #[error("VoltageNotCalibrated")]
    VoltageNotCalibrated = -201496,
    #[error("MissingRangeForCalibration")]
    MissingRangeForCalibration = -201495,
    #[error("MultipleChansNotSupportedDuringCalAdjust")]
    MultipleChansNotSupportedDuringCalAdjust = -201494,
    #[error("ShuntCalFailedOutOfRange")]
    ShuntCalFailedOutOfRange = -201493,
    #[error("OperationNotSupportedOnSimulatedDevice")]
    OperationNotSupportedOnSimulatedDevice = -201492,
    #[error("FirmwareVersionSameAsInstalledVersion")]
    FirmwareVersionSameAsInstalledVersion = -201491,
    #[error("FirmwareVersionOlderThanInstalledVersion")]
    FirmwareVersionOlderThanInstalledVersion = -201490,
    #[error("FirmwareUpdateInvalidState")]
    FirmwareUpdateInvalidState = -201489,
    #[error("FirmwareUpdateInvalidID")]
    FirmwareUpdateInvalidID = -201488,
    #[error("FirmwareUpdateAutomaticManagementEnabled")]
    FirmwareUpdateAutomaticManagementEnabled = -201487,
    #[error("SetupCalibrationNotCalled")]
    SetupCalibrationNotCalled = -201486,
    #[error("CalMeasuredDataSizeVsActualDataSizeMismatch")]
    CalMeasuredDataSizeVsActualDataSizeMismatch = -201485,
    #[error("CDAQMissingDSAMasterForChanExpansion")]
    CDAQMissingDSAMasterForChanExpansion = -201484,
    #[error("CDAQMasterNotFoundForChanExpansion")]
    CDAQMasterNotFoundForChanExpansion = -201483,
    #[error("AllChansShouldBeProvidedForCalibration")]
    AllChansShouldBeProvidedForCalibration = -201482,
    #[error("MustSpecifyExpirationStateForAllLinesInRange")]
    MustSpecifyExpirationStateForAllLinesInRange = -201481,
    #[error("OpenSessionExists")]
    OpenSessionExists = -201480,
    #[error("CannotQueryTerminalForSWArmStart")]
    CannotQueryTerminalForSWArmStart = -201479,
    #[error("ChassisWatchdogTimerExpired")]
    ChassisWatchdogTimerExpired = -201478,
    #[error("CantReserveWatchdogTaskWhileOtherTasksReserved")]
    CantReserveWatchdogTaskWhileOtherTasksReserved = -201477,
    #[error("CantReserveTaskWhileWatchdogTaskReserving")]
    CantReserveTaskWhileWatchdogTaskReserving = -201476,
    #[error("AuxPowerSourceRequired")]
    AuxPowerSourceRequired = -201475,
    #[error("DeviceNotSupportedOnLocalSystem")]
    DeviceNotSupportedOnLocalSystem = -201474,
    #[error("OneTimestampChannelRequiredForCombinedNavigationRead")]
    OneTimestampChannelRequiredForCombinedNavigationRead = -201472,
    #[error("MultDevsMultPhysChans")]
    MultDevsMultPhysChans = -201471,
    #[error("InvalidCalAdjustmentPointValues")]
    InvalidCalAdjustmentPointValues = -201470,
    #[error("DifferentDigitizerFromCommunicator")]
    DifferentDigitizerFromCommunicator = -201469,
    #[error("CDAQSyncMasterClockNotPresent")]
    CDAQSyncMasterClockNotPresent = -201468,
    #[error("AssociatedChansHaveConflictingProps")]
    AssociatedChansHaveConflictingProps = -201467,
    #[error("AutoConfigBetweenMultipleDeviceStatesInvalid")]
    AutoConfigBetweenMultipleDeviceStatesInvalid = -201466,
    #[error("AutoConfigOfOfflineDevicesInvalid")]
    AutoConfigOfOfflineDevicesInvalid = -201465,
    #[error("ExternalFIFOFault")]
    ExternalFIFOFault = -201464,
    #[error("ConnectionsNotReciprocal")]
    ConnectionsNotReciprocal = -201463,
    #[error("InvalidOutputToInputCDAQSyncConnection")]
    InvalidOutputToInputCDAQSyncConnection = -201462,
    #[error("ReferenceClockNotPresent")]
    ReferenceClockNotPresent = -201461,
    #[error("BlankStringExpansionFoundNoSupportedCDAQSyncConnectionDevices")]
    BlankStringExpansionFoundNoSupportedCDAQSyncConnectionDevices = -201460,
    #[error("NoDevicesSupportCDAQSyncConnections")]
    NoDevicesSupportCDAQSyncConnections = -201459,
    #[error("InvalidCDAQSyncTimeoutValue")]
    InvalidCDAQSyncTimeoutValue = -201458,
    #[error("CDAQSyncConnectionToSamePort")]
    CDAQSyncConnectionToSamePort = -201457,
    #[error("DevsWithoutCommonSyncConnectionStrategy")]
    DevsWithoutCommonSyncConnectionStrategy = -201456,
    #[error("NoCDAQSyncBetweenPhysAndSimulatedDevs")]
    NoCDAQSyncBetweenPhysAndSimulatedDevs = -201455,
    #[error("UnableToContainCards")]
    UnableToContainCards = -201454,
    #[error("FindDisconnectedBetweenPhysAndSimDeviceStatesInvalid")]
    FindDisconnectedBetweenPhysAndSimDeviceStatesInvalid = -201453,
    #[error("OperationAborted")]
    OperationAborted = -201452,
    #[error("TwoPortsRequired")]
    TwoPortsRequired = -201451,
    #[error("DeviceDoesNotSupportCDAQSyncConnections")]
    DeviceDoesNotSupportCDAQSyncConnections = -201450,
    #[error("InvalidcDAQSyncPortConnectionFormat")]
    InvalidcDAQSyncPortConnectionFormat = -201449,
    #[error("RosetteMeasurementsNotSpecified")]
    RosetteMeasurementsNotSpecified = -201448,
    #[error("InvalidNumOfPhysChansForDeltaRosette")]
    InvalidNumOfPhysChansForDeltaRosette = -201447,
    #[error("InvalidNumOfPhysChansForTeeRosette")]
    InvalidNumOfPhysChansForTeeRosette = -201446,
    #[error("RosetteStrainChanNamesNeeded")]
    RosetteStrainChanNamesNeeded = -201445,
    #[error("MultideviceWithOnDemandTiming")]
    MultideviceWithOnDemandTiming = -201444,
    #[error("FREQOUTCannotProduceDesiredFrequency3")]
    FREQOUTCannotProduceDesiredFrequency3 = -201443,
    #[error("TwoEdgeSeparationSameTerminalSameEdge")]
    TwoEdgeSeparationSameTerminalSameEdge = -201442,
    #[error("DontMixSyncPulseAndSampClkTimebaseOn449x")]
    DontMixSyncPulseAndSampClkTimebaseOn449x = -201441,
    #[error("NeitherRefClkNorSampClkTimebaseConfiguredForDSASync")]
    NeitherRefClkNorSampClkTimebaseConfiguredForDSASync = -201440,
    #[error("RetriggeringFiniteCONotAllowed")]
    RetriggeringFiniteCONotAllowed = -201439,
    #[error("DeviceRebootedFromWDTTimeout")]
    DeviceRebootedFromWDTTimeout = -201438,
    #[error("TimeoutValueExceedsMaximum")]
    TimeoutValueExceedsMaximum = -201437,
    #[error("SharingDifferentWireModes")]
    SharingDifferentWireModes = -201436,
    #[error("CantPrimeWithEmptyBuffer")]
    CantPrimeWithEmptyBuffer = -201435,
    #[error("ConfigFailedBecauseWatchdogExpired")]
    ConfigFailedBecauseWatchdogExpired = -201434,
    #[error("WriteFailedBecauseWatchdogChangedLineDirection")]
    WriteFailedBecauseWatchdogChangedLineDirection = -201433,
    #[error("MultipleSubsytemCalibration")]
    MultipleSubsytemCalibration = -201432,
    #[error("IncorrectChannelForOffsetAdjustment")]
    IncorrectChannelForOffsetAdjustment = -201431,
    #[error("InvalidNumRefVoltagesToWrite")]
    InvalidNumRefVoltagesToWrite = -201430,
    #[error("StartTrigDelayWithDSAModule")]
    StartTrigDelayWithDSAModule = -201429,
    #[error("MoreThanOneSyncPulseDetected")]
    MoreThanOneSyncPulseDetected = -201428,
    #[error("DevNotSupportedWithinDAQmxAPI")]
    DevNotSupportedWithinDAQmxAPI = -201427,
    #[error("DevsWithoutSyncStrategies")]
    DevsWithoutSyncStrategies = -201426,
    #[error("DevsWithoutCommonSyncStrategy")]
    DevsWithoutCommonSyncStrategy = -201425,
    #[error("SyncStrategiesCannotSync")]
    SyncStrategiesCannotSync = -201424,
    #[error("ChassisCommunicationInterrupted")]
    ChassisCommunicationInterrupted = -201423,
    #[error("UnknownCardPowerProfileInCarrier")]
    UnknownCardPowerProfileInCarrier = -201422,
    #[error("AttrNotSupportedOnAccessory")]
    AttrNotSupportedOnAccessory = -201421,
    #[error("NetworkDeviceReservedByAnotherHost")]
    NetworkDeviceReservedByAnotherHost = -201420,
    #[error("IncorrectFirmwareFileUploaded")]
    IncorrectFirmwareFileUploaded = -201419,
    #[error("InvalidFirmwareFileUploaded")]
    InvalidFirmwareFileUploaded = -201418,
    #[error("InTimerTimeoutOnArm")]
    InTimerTimeoutOnArm = -201417,
    #[error("CantExceedSlotRelayDriveLimit")]
    CantExceedSlotRelayDriveLimit = -201416,
    #[error("ModuleUnsupportedFor9163")]
    ModuleUnsupportedFor9163 = -201415,
    #[error("ConnectionsNotSupported")]
    ConnectionsNotSupported = -201414,
    #[error("AccessoryNotPresent")]
    AccessoryNotPresent = -201413,
    #[error("SpecifiedAccessoryChannelsNotPresentOnDevice")]
    SpecifiedAccessoryChannelsNotPresentOnDevice = -201412,
    #[error("ConnectionsNotSupportedOnAccessory")]
    ConnectionsNotSupportedOnAccessory = -201411,
    #[error("RateTooFastForHWTSP")]
    RateTooFastForHWTSP = -201410,
    #[error("DelayFromSampleClockOutOfRangeForHWTSP")]
    DelayFromSampleClockOutOfRangeForHWTSP = -201409,
    #[error("AveragingWhenNotInternalHWTSP")]
    AveragingWhenNotInternalHWTSP = -201408,
    #[error("AttributeNotSupportedUnlessHWTSP")]
    AttributeNotSupportedUnlessHWTSP = -201407,
    #[error("FiveVoltDetectFailed")]
    FiveVoltDetectFailed = -201406,
    #[error("AnalogBusStateInconsistent")]
    AnalogBusStateInconsistent = -201405,
    #[error("CardDetectedDoesNotMatchExpectedCard")]
    CardDetectedDoesNotMatchExpectedCard = -201404,
    #[error("LoggingStartNewFileNotCalled")]
    LoggingStartNewFileNotCalled = -201403,
    #[error("LoggingSampsPerFileNotDivisible")]
    LoggingSampsPerFileNotDivisible = -201402,
    #[error("RetrievingNetworkDeviceProperties")]
    RetrievingNetworkDeviceProperties = -201401,
    #[error("FilePreallocationFailed")]
    FilePreallocationFailed = -201400,
    #[error("ModuleMismatchInSameTimedTask")]
    ModuleMismatchInSameTimedTask = -201399,
    #[error("InvalidAttributeValuePossiblyDueToOtherAttributeValues")]
    InvalidAttributeValuePossiblyDueToOtherAttributeValues = -201398,
    #[error("ChangeDetectionStoppedToPreventDeviceHang")]
    ChangeDetectionStoppedToPreventDeviceHang = -201397,
    #[error("FilterDelayRemovalNotPosssibleWithAnalogTrigger")]
    FilterDelayRemovalNotPosssibleWithAnalogTrigger = -201396,
    #[error("NonbufferedOrNoChannels")]
    NonbufferedOrNoChannels = -201395,
    #[error("TristateLogicLevelNotSpecdForEntirePort")]
    TristateLogicLevelNotSpecdForEntirePort = -201394,
    #[error("TristateLogicLevelNotSupportedOnDigOutChan")]
    TristateLogicLevelNotSupportedOnDigOutChan = -201393,
    #[error("TristateLogicLevelNotSupported")]
    TristateLogicLevelNotSupported = -201392,
    #[error("IncompleteGainAndCouplingCalAdjustment")]
    IncompleteGainAndCouplingCalAdjustment = -201391,
    #[error("NetworkStatusConnectionLost")]
    NetworkStatusConnectionLost = -201390,
    #[error("ModuleChangeDuringConnectionLoss")]
    ModuleChangeDuringConnectionLoss = -201389,
    #[error("NetworkDeviceNotReservedByHost")]
    NetworkDeviceNotReservedByHost = -201388,
    #[error("DuplicateCalibrationAdjustmentInput")]
    DuplicateCalibrationAdjustmentInput = -201387,
    #[error("SelfCalFailedContactTechSupport")]
    SelfCalFailedContactTechSupport = -201386,
    #[error("SelfCalFailedToConverge")]
    SelfCalFailedToConverge = -201385,
    #[error("UnsupportedSimulatedModuleForSimulatedChassis")]
    UnsupportedSimulatedModuleForSimulatedChassis = -201384,
    #[error("LoggingWriteSizeTooBig")]
    LoggingWriteSizeTooBig = -201383,
    #[error("LoggingWriteSizeNotDivisible")]
    LoggingWriteSizeNotDivisible = -201382,
    #[error("MyDAQPowerRailFault")]
    MyDAQPowerRailFault = -201381,
    #[error("DeviceDoesNotSupportThisOperation")]
    DeviceDoesNotSupportThisOperation = -201380,
    #[error("NetworkDevicesNotSupportedOnThisPlatform")]
    NetworkDevicesNotSupportedOnThisPlatform = -201379,
    #[error("UnknownFirmwareVersion")]
    UnknownFirmwareVersion = -201378,
    #[error("FirmwareIsUpdating")]
    FirmwareIsUpdating = -201377,
    #[error("AccessoryEEPROMIsCorrupt")]
    AccessoryEEPROMIsCorrupt = -201376,
    #[error("ThrmcplLeadOffsetNullingCalNotSupported")]
    ThrmcplLeadOffsetNullingCalNotSupported = -201375,
    #[error("SelfCalFailedTryExtCal")]
    SelfCalFailedTryExtCal = -201374,
    #[error("OutputP2PNotSupportedWithMultithreadedScripts")]
    OutputP2PNotSupportedWithMultithreadedScripts = -201373,
    #[error("ThrmcplCalibrationChannelsOpen")]
    ThrmcplCalibrationChannelsOpen = -201372,
    #[error("MDNSServiceInstanceAlreadyInUse")]
    MDNSServiceInstanceAlreadyInUse = -201371,
    #[error("IPAddressAlreadyInUse")]
    IPAddressAlreadyInUse = -201370,
    #[error("HostnameAlreadyInUse")]
    HostnameAlreadyInUse = -201369,
    #[error("InvalidNumberOfCalAdjustmentPoints")]
    InvalidNumberOfCalAdjustmentPoints = -201368,
    #[error("FilterOrDigitalSyncInternalSignal")]
    FilterOrDigitalSyncInternalSignal = -201367,
    #[error("BadDDSSource")]
    BadDDSSource = -201366,
    #[error("OnboardRegenWithMoreThan16Channels")]
    OnboardRegenWithMoreThan16Channels = -201365,
    #[error("TriggerTooFast")]
    TriggerTooFast = -201364,
    #[error("MinMaxOutsideTableRange")]
    MinMaxOutsideTableRange = -201363,
    #[error("ChannelExpansionWithInvalidAnalogTriggerDevice")]
    ChannelExpansionWithInvalidAnalogTriggerDevice = -201362,
    #[error("SyncPulseSrcInvalidForTask")]
    SyncPulseSrcInvalidForTask = -201361,
    #[error("InvalidCarrierSlotNumberSpecd")]
    InvalidCarrierSlotNumberSpecd = -201360,
    #[error("CardsMustBeInSameCarrier")]
    CardsMustBeInSameCarrier = -201359,
    #[error("CardDevCarrierSimMustMatch")]
    CardDevCarrierSimMustMatch = -201358,
    #[error("DevMustHaveAtLeastOneCard")]
    DevMustHaveAtLeastOneCard = -201357,
    #[error("CardTopologyError")]
    CardTopologyError = -201356,
    #[error("ExceededCarrierPowerLimit")]
    ExceededCarrierPowerLimit = -201355,
    #[error("CardsIncompatible")]
    CardsIncompatible = -201354,
    #[error("AnalogBusNotValid")]
    AnalogBusNotValid = -201353,
    #[error("ReservationConflict")]
    ReservationConflict = -201352,
    #[error("MemMappedOnDemandNotSupported")]
    MemMappedOnDemandNotSupported = -201351,
    #[error("SlaveWithNoStartTriggerConfigured")]
    SlaveWithNoStartTriggerConfigured = -201350,
    #[error("ChannelExpansionWithDifferentTriggerDevices")]
    ChannelExpansionWithDifferentTriggerDevices = -201349,
    #[error("CounterSyncAndRetriggered")]
    CounterSyncAndRetriggered = -201348,
    #[error("NoExternalSyncPulseDetected")]
    NoExternalSyncPulseDetected = -201347,
    #[error("SlaveAndNoExternalSyncPulse")]
    SlaveAndNoExternalSyncPulse = -201346,
    #[error("CustomTimingRequiredForAttribute")]
    CustomTimingRequiredForAttribute = -201345,
    #[error("CustomTimingModeNotSet")]
    CustomTimingModeNotSet = -201344,
    #[error("AccessoryPowerTripped")]
    AccessoryPowerTripped = -201343,
    #[error("UnsupportedAccessory")]
    UnsupportedAccessory = -201342,
    #[error("InvalidAccessoryChange")]
    InvalidAccessoryChange = -201341,
    #[error("FirmwareRequiresUpgrade")]
    FirmwareRequiresUpgrade = -201340,
    #[error("FastExternalTimebaseNotSupportedForDevice")]
    FastExternalTimebaseNotSupportedForDevice = -201339,
    #[error("InvalidShuntLocationForCalibration")]
    InvalidShuntLocationForCalibration = -201338,
    #[error("DeviceNameTooLong")]
    DeviceNameTooLong = -201337,
    #[error("BridgeScalesUnsupported")]
    BridgeScalesUnsupported = -201336,
    #[error("MismatchedElecPhysValues")]
    MismatchedElecPhysValues = -201335,
    #[error("LinearRequiresUniquePoints")]
    LinearRequiresUniquePoints = -201334,
    #[error("MissingRequiredScalingParameter")]
    MissingRequiredScalingParameter = -201333,
    #[error("LoggingNotSupportOnOutputTasks")]
    LoggingNotSupportOnOutputTasks = -201332,
    #[error("MemoryMappedHardwareTimedNonBufferedUnsupported")]
    MemoryMappedHardwareTimedNonBufferedUnsupported = -201331,
    #[error("CannotUpdatePulseTrainWithAutoIncrementEnabled")]
    CannotUpdatePulseTrainWithAutoIncrementEnabled = -201330,
    #[error("HWTimedSinglePointAndDataXferNotDMA")]
    HWTimedSinglePointAndDataXferNotDMA = -201329,
    #[error("SCCSecondStageEmpty")]
    SCCSecondStageEmpty = -201328,
    #[error("SCCInvalidDualStageCombo")]
    SCCInvalidDualStageCombo = -201327,
    #[error("SCCInvalidSecondStage")]
    SCCInvalidSecondStage = -201326,
    #[error("SCCInvalidFirstStage")]
    SCCInvalidFirstStage = -201325,
    #[error("CounterMultipleSampleClockedChannels")]
    CounterMultipleSampleClockedChannels = -201324,
    #[error("TwoCounterMeasurementModeAndSampleClocked")]
    TwoCounterMeasurementModeAndSampleClocked = -201323,
    #[error("CantHaveBothMemMappedAndNonMemMappedTasks")]
    CantHaveBothMemMappedAndNonMemMappedTasks = -201322,
    #[error("MemMappedDataReadByAnotherProcess")]
    MemMappedDataReadByAnotherProcess = -201321,
    #[error("RetriggeringInvalidForGivenSettings")]
    RetriggeringInvalidForGivenSettings = -201320,
    #[error("AIOverrun")]
    AIOverrun = -201319,
    #[error("COOverrun")]
    COOverrun = -201318,
    #[error("CounterMultipleBufferedChannels")]
    CounterMultipleBufferedChannels = -201317,
    #[error("InvalidTimebaseForCOHWTSP")]
    InvalidTimebaseForCOHWTSP = -201316,
    #[error("WriteBeforeEvent")]
    WriteBeforeEvent = -201315,
    #[error("CIOverrun")]
    CIOverrun = -201314,
    #[error("CounterNonResponsiveAndReset")]
    CounterNonResponsiveAndReset = -201313,
    #[error("MeasTypeOrChannelNotSupportedForLogging")]
    MeasTypeOrChannelNotSupportedForLogging = -201312,
    #[error("FileAlreadyOpenedForWrite")]
    FileAlreadyOpenedForWrite = -201311,
    #[error("TdmsNotFound")]
    TdmsNotFound = -201310,
    #[error("GenericFileIO")]
    GenericFileIO = -201309,
    #[error("FiniteSTCCounterNotSupportedForLogging")]
    FiniteSTCCounterNotSupportedForLogging = -201308,
    #[error("MeasurementTypeNotSupportedForLogging")]
    MeasurementTypeNotSupportedForLogging = -201307,
    #[error("FileAlreadyOpened")]
    FileAlreadyOpened = -201306,
    #[error("DiskFull")]
    DiskFull = -201305,
    #[error("FilePathInvalid")]
    FilePathInvalid = -201304,
    #[error("FileVersionMismatch")]
    FileVersionMismatch = -201303,
    #[error("FileWriteProtected")]
    FileWriteProtected = -201302,
    #[error("ReadNotSupportedForLoggingMode")]
    ReadNotSupportedForLoggingMode = -201301,
    #[error("AttributeNotSupportedWhenLogging")]
    AttributeNotSupportedWhenLogging = -201300,
    #[error("LoggingModeNotSupportedNonBuffered")]
    LoggingModeNotSupportedNonBuffered = -201299,
    #[error("PropertyNotSupportedWithConflictingProperty")]
    PropertyNotSupportedWithConflictingProperty = -201298,
    #[error("ParallelSSHOnConnector1")]
    ParallelSSHOnConnector1 = -201297,
    #[error("COOnlyImplicitSampleTimingTypeSupported")]
    COOnlyImplicitSampleTimingTypeSupported = -201296,
    #[error("CalibrationFailedAOOutOfRange")]
    CalibrationFailedAOOutOfRange = -201295,
    #[error("CalibrationFailedAIOutOfRange")]
    CalibrationFailedAIOutOfRange = -201294,
    #[error("CalPWMLinearityFailed")]
    CalPWMLinearityFailed = -201293,
    #[error("OverrunUnderflowConfigurationCombo")]
    OverrunUnderflowConfigurationCombo = -201292,
    #[error("CannotWriteToFiniteCOTask")]
    CannotWriteToFiniteCOTask = -201291,
    #[error("NetworkDAQInvalidWEPKeyLength")]
    NetworkDAQInvalidWEPKeyLength = -201290,
    #[error("CalInputsShortedNotSupported")]
    CalInputsShortedNotSupported = -201289,
    #[error("CannotSetPropertyWhenTaskIsReserved")]
    CannotSetPropertyWhenTaskIsReserved = -201288,
    #[error("Minus12VFuseBlown")]
    Minus12VFuseBlown = -201287,
    #[error("Plus12VFuseBlown")]
    Plus12VFuseBlown = -201286,
    #[error("Plus5VFuseBlown")]
    Plus5VFuseBlown = -201285,
    #[error("Plus3VFuseBlown")]
    Plus3VFuseBlown = -201284,
    #[error("DeviceSerialPortError")]
    DeviceSerialPortError = -201283,
    #[error("PowerUpStateMachineNotDone")]
    PowerUpStateMachineNotDone = -201282,
    #[error("TooManyTriggersSpecifiedInTask")]
    TooManyTriggersSpecifiedInTask = -201281,
    #[error("VerticalOffsetNotSupportedOnDevice")]
    VerticalOffsetNotSupportedOnDevice = -201280,
    #[error("InvalidCouplingForMeasurementType")]
    InvalidCouplingForMeasurementType = -201279,
    #[error("DigitalLineUpdateTooFastForDevice")]
    DigitalLineUpdateTooFastForDevice = -201278,
    #[error("CertificateIsTooBigToTransfer")]
    CertificateIsTooBigToTransfer = -201277,
    #[error("OnlyPEMOrDERCertiticatesAccepted")]
    OnlyPEMOrDERCertiticatesAccepted = -201276,
    #[error("CalCouplingNotSupported")]
    CalCouplingNotSupported = -201275,
    #[error("DeviceNotSupportedIn64Bit")]
    DeviceNotSupportedIn64Bit = -201274,
    #[error("NetworkDeviceInUse")]
    NetworkDeviceInUse = -201273,
    #[error("InvalidIPv4AddressFormat")]
    InvalidIPv4AddressFormat = -201272,
    #[error("NetworkProductTypeMismatch")]
    NetworkProductTypeMismatch = -201271,
    #[error("OnlyPEMCertificatesAccepted")]
    OnlyPEMCertificatesAccepted = -201270,
    #[error("CalibrationRequiresPrototypingBoardEnabled")]
    CalibrationRequiresPrototypingBoardEnabled = -201269,
    #[error("AllCurrentLimitingResourcesAlreadyTaken")]
    AllCurrentLimitingResourcesAlreadyTaken = -201268,
    #[error("UserDefInfoStringBadLength")]
    UserDefInfoStringBadLength = -201267,
    #[error("PropertyNotFound")]
    PropertyNotFound = -201266,
    #[error("OverVoltageProtectionActivated")]
    OverVoltageProtectionActivated = -201265,
    #[error("ScaledIQWaveformTooLarge")]
    ScaledIQWaveformTooLarge = -201264,
    #[error("FirmwareFailedToDownload")]
    FirmwareFailedToDownload = -201263,
    #[error("PropertyNotSupportedForBusType")]
    PropertyNotSupportedForBusType = -201262,
    #[error("ChangeRateWhileRunningCouldNotBeCompleted")]
    ChangeRateWhileRunningCouldNotBeCompleted = -201261,
    #[error("CannotQueryManualControlAttribute")]
    CannotQueryManualControlAttribute = -201260,
    #[error("InvalidNetworkConfiguration")]
    InvalidNetworkConfiguration = -201259,
    #[error("InvalidWirelessConfiguration")]
    InvalidWirelessConfiguration = -201258,
    #[error("InvalidWirelessCountryCode")]
    InvalidWirelessCountryCode = -201257,
    #[error("InvalidWirelessChannel")]
    InvalidWirelessChannel = -201256,
    #[error("NetworkEEPROMHasChanged")]
    NetworkEEPROMHasChanged = -201255,
    #[error("NetworkSerialNumberMismatch")]
    NetworkSerialNumberMismatch = -201254,
    #[error("NetworkStatusDown")]
    NetworkStatusDown = -201253,
    #[error("NetworkTargetUnreachable")]
    NetworkTargetUnreachable = -201252,
    #[error("NetworkTargetNotFound")]
    NetworkTargetNotFound = -201251,
    #[error("NetworkStatusTimedOut")]
    NetworkStatusTimedOut = -201250,
    #[error("InvalidWirelessSecuritySelection")]
    InvalidWirelessSecuritySelection = -201249,
    #[error("NetworkDeviceConfigurationLocked")]
    NetworkDeviceConfigurationLocked = -201248,
    #[error("NetworkDAQDeviceNotSupported")]
    NetworkDAQDeviceNotSupported = -201247,
    #[error("NetworkDAQCannotCreateEmptySleeve")]
    NetworkDAQCannotCreateEmptySleeve = -201246,
    #[error("UserDefInfoStringTooLong")]
    UserDefInfoStringTooLong = -201245,
    #[error("ModuleTypeDoesNotMatchModuleTypeInDestination")]
    ModuleTypeDoesNotMatchModuleTypeInDestination = -201244,
    #[error("InvalidTEDSInterfaceAddress")]
    InvalidTEDSInterfaceAddress = -201243,
    #[error("DevDoesNotSupportSCXIComm")]
    DevDoesNotSupportSCXIComm = -201242,
    #[error("SCXICommDevConnector0MustBeCabledToModule")]
    SCXICommDevConnector0MustBeCabledToModule = -201241,
    #[error("SCXIModuleDoesNotSupportDigitizationMode")]
    SCXIModuleDoesNotSupportDigitizationMode = -201240,
    #[error("DevDoesNotSupportMultiplexedSCXIDigitizationMode")]
    DevDoesNotSupportMultiplexedSCXIDigitizationMode = -201239,
    #[error("DevOrDevPhysChanDoesNotSupportSCXIDigitization")]
    DevOrDevPhysChanDoesNotSupportSCXIDigitization = -201238,
    #[error("InvalidPhysChanName")]
    InvalidPhysChanName = -201237,
    #[error("SCXIChassisCommModeInvalid")]
    SCXIChassisCommModeInvalid = -201236,
    #[error("RequiredDependencyNotFound")]
    RequiredDependencyNotFound = -201235,
    #[error("InvalidStorage")]
    InvalidStorage = -201234,
    #[error("InvalidObject")]
    InvalidObject = -201233,
    #[error("StorageAlteredPriorToSave")]
    StorageAlteredPriorToSave = -201232,
    #[error("TaskDoesNotReferenceLocalChannel")]
    TaskDoesNotReferenceLocalChannel = -201231,
    #[error("ReferencedDevSimMustMatchTarget")]
    ReferencedDevSimMustMatchTarget = -201230,
    #[error("ProgrammedIOFailsBecauseOfWatchdogTimer")]
    ProgrammedIOFailsBecauseOfWatchdogTimer = -201229,
    #[error("WatchdogTimerFailsBecauseOfProgrammedIO")]
    WatchdogTimerFailsBecauseOfProgrammedIO = -201228,
    #[error("CantUseThisTimingEngineWithAPort")]
    CantUseThisTimingEngineWithAPort = -201227,
    #[error("ProgrammedIOConflict")]
    ProgrammedIOConflict = -201226,
    #[error("ChangeDetectionIncompatibleWithProgrammedIO")]
    ChangeDetectionIncompatibleWithProgrammedIO = -201225,
    #[error("TristateNotEnoughLines")]
    TristateNotEnoughLines = -201224,
    #[error("TristateConflict")]
    TristateConflict = -201223,
    #[error("GenerateOrFiniteWaitExpectedBeforeBreakBlock")]
    GenerateOrFiniteWaitExpectedBeforeBreakBlock = -201222,
    #[error("BreakBlockNotAllowedInLoop")]
    BreakBlockNotAllowedInLoop = -201221,
    #[error("ClearTriggerNotAllowedInBreakBlock")]
    ClearTriggerNotAllowedInBreakBlock = -201220,
    #[error("NestingNotAllowedInBreakBlock")]
    NestingNotAllowedInBreakBlock = -201219,
    #[error("IfElseBlockNotAllowedInBreakBlock")]
    IfElseBlockNotAllowedInBreakBlock = -201218,
    #[error("RepeatUntilTriggerLoopNotAllowedInBreakBlock")]
    RepeatUntilTriggerLoopNotAllowedInBreakBlock = -201217,
    #[error("WaitUntilTriggerNotAllowedInBreakBlock")]
    WaitUntilTriggerNotAllowedInBreakBlock = -201216,
    #[error("MarkerPosInvalidInBreakBlock")]
    MarkerPosInvalidInBreakBlock = -201215,
    #[error("InvalidWaitDurationInBreakBlock")]
    InvalidWaitDurationInBreakBlock = -201214,
    #[error("InvalidSubsetLengthInBreakBlock")]
    InvalidSubsetLengthInBreakBlock = -201213,
    #[error("InvalidWaveformLengthInBreakBlock")]
    InvalidWaveformLengthInBreakBlock = -201212,
    #[error("InvalidWaitDurationBeforeBreakBlock")]
    InvalidWaitDurationBeforeBreakBlock = -201211,
    #[error("InvalidSubsetLengthBeforeBreakBlock")]
    InvalidSubsetLengthBeforeBreakBlock = -201210,
    #[error("InvalidWaveformLengthBeforeBreakBlock")]
    InvalidWaveformLengthBeforeBreakBlock = -201209,
    #[error("SampleRateTooHighForADCTimingMode")]
    SampleRateTooHighForADCTimingMode = -201208,
    #[error("ActiveDevNotSupportedWithMultiDevTask")]
    ActiveDevNotSupportedWithMultiDevTask = -201207,
    #[error("RealDevAndSimDevNotSupportedInSameTask")]
    RealDevAndSimDevNotSupportedInSameTask = -201206,
    #[error("RTSISimMustMatchDevSim")]
    RTSISimMustMatchDevSim = -201205,
    #[error("BridgeShuntCaNotSupported")]
    BridgeShuntCaNotSupported = -201204,
    #[error("StrainShuntCaNotSupported")]
    StrainShuntCaNotSupported = -201203,
    #[error("GainTooLargeForGainCalConst")]
    GainTooLargeForGainCalConst = -201202,
    #[error("OffsetTooLargeForOffsetCalConst")]
    OffsetTooLargeForOffsetCalConst = -201201,
    #[error("ElvisPrototypingBoardRemoved")]
    ElvisPrototypingBoardRemoved = -201200,
    #[error("Elvis2PowerRailFault")]
    Elvis2PowerRailFault = -201199,
    #[error("Elvis2PhysicalChansFault")]
    Elvis2PhysicalChansFault = -201198,
    #[error("Elvis2PhysicalChansThermalEvent")]
    Elvis2PhysicalChansThermalEvent = -201197,
    #[error("RXBitErrorRateLimitExceeded")]
    RXBitErrorRateLimitExceeded = -201196,
    #[error("PHYBitErrorRateLimitExceeded")]
    PHYBitErrorRateLimitExceeded = -201195,
    #[error("TwoPartAttributeCalledOutOfOrder")]
    TwoPartAttributeCalledOutOfOrder = -201194,
    #[error("InvalidSCXIChassisAddress")]
    InvalidSCXIChassisAddress = -201193,
    #[error("CouldNotConnectToRemoteMXS")]
    CouldNotConnectToRemoteMXS = -201192,
    #[error("ExcitationStateRequiredForAttributes")]
    ExcitationStateRequiredForAttributes = -201191,
    #[error("DeviceNotUsableUntilUSBReplug")]
    DeviceNotUsableUntilUSBReplug = -201190,
    #[error("InputFIFOOverflowDuringCalibrationOnFullSpeedUSB")]
    InputFIFOOverflowDuringCalibrationOnFullSpeedUSB = -201189,
    #[error("InputFIFOOverflowDuringCalibration")]
    InputFIFOOverflowDuringCalibration = -201188,
    #[error("CJCChanConflictsWithNonThermocoupleChan")]
    CJCChanConflictsWithNonThermocoupleChan = -201187,
    #[error("CommDeviceForPXIBackplaneNotInRightmostSlot")]
    CommDeviceForPXIBackplaneNotInRightmostSlot = -201186,
    #[error("CommDeviceForPXIBackplaneNotInSameChassis")]
    CommDeviceForPXIBackplaneNotInSameChassis = -201185,
    #[error("CommDeviceForPXIBackplaneNotPXI")]
    CommDeviceForPXIBackplaneNotPXI = -201184,
    #[error("InvalidCalExcitFrequency")]
    InvalidCalExcitFrequency = -201183,
    #[error("InvalidCalExcitVoltage")]
    InvalidCalExcitVoltage = -201182,
    #[error("InvalidAIInputSrc")]
    InvalidAIInputSrc = -201181,
    #[error("InvalidCalInputRef")]
    InvalidCalInputRef = -201180,
    #[error("DBReferenceValueNotGreaterThanZero")]
    DBReferenceValueNotGreaterThanZero = -201179,
    #[error("SampleClockRateIsTooFastForSampleClockTiming")]
    SampleClockRateIsTooFastForSampleClockTiming = -201178,
    #[error("DeviceNotUsableUntilColdStart")]
    DeviceNotUsableUntilColdStart = -201177,
    #[error("SampleClockRateIsTooFastForBurstTiming")]
    SampleClockRateIsTooFastForBurstTiming = -201176,
    #[error("DevImportFailedAssociatedResourceIDsNotSupported")]
    DevImportFailedAssociatedResourceIDsNotSupported = -201175,
    #[error("SCXI1600ImportNotSupported")]
    SCXI1600ImportNotSupported = -201174,
    #[error("PowerSupplyConfigurationFailed")]
    PowerSupplyConfigurationFailed = -201173,
    #[error("IEPEWithDCNotAllowed")]
    IEPEWithDCNotAllowed = -201172,
    #[error("MinTempForThermocoupleTypeOutsideAccuracyForPolyScaling")]
    MinTempForThermocoupleTypeOutsideAccuracyForPolyScaling = -201171,
    #[error("DevImportFailedNoDeviceToOverwriteAndSimulationNotSupported")]
    DevImportFailedNoDeviceToOverwriteAndSimulationNotSupported = -201170,
    #[error("DevImportFailedDeviceNotSupportedOnDestination")]
    DevImportFailedDeviceNotSupportedOnDestination = -201169,
    #[error("FirmwareIsTooOld")]
    FirmwareIsTooOld = -201168,
    #[error("FirmwareCouldntUpdate")]
    FirmwareCouldntUpdate = -201167,
    #[error("FirmwareIsCorrupt")]
    FirmwareIsCorrupt = -201166,
    #[error("FirmwareTooNew")]
    FirmwareTooNew = -201165,
    #[error("SampClockCannotBeExportedFromExternalSampClockSrc")]
    SampClockCannotBeExportedFromExternalSampClockSrc = -201164,
    #[error("PhysChanReservedForInputWhenDesiredForOutput")]
    PhysChanReservedForInputWhenDesiredForOutput = -201163,
    #[error("PhysChanReservedForOutputWhenDesiredForInput")]
    PhysChanReservedForOutputWhenDesiredForInput = -201162,
    #[error("SpecifiedCDAQSlotNotEmpty")]
    SpecifiedCDAQSlotNotEmpty = -201161,
    #[error("DeviceDoesNotSupportSimulation")]
    DeviceDoesNotSupportSimulation = -201160,
    #[error("InvalidCDAQSlotNumberSpecd")]
    InvalidCDAQSlotNumberSpecd = -201159,
    #[error("CSeriesModSimMustMatchCDAQChassisSim")]
    CSeriesModSimMustMatchCDAQChassisSim = -201158,
    #[error("SCCCabledDevMustNotBeSimWhenSCCCarrierIsNotSim")]
    SCCCabledDevMustNotBeSimWhenSCCCarrierIsNotSim = -201157,
    #[error("SCCModSimMustMatchSCCCarrierSim")]
    SCCModSimMustMatchSCCCarrierSim = -201156,
    #[error("SCXIModuleDoesNotSupportSimulation")]
    SCXIModuleDoesNotSupportSimulation = -201155,
    #[error("SCXICableDevMustNotBeSimWhenModIsNotSim")]
    SCXICableDevMustNotBeSimWhenModIsNotSim = -201154,
    #[error("SCXIDigitizerSimMustNotBeSimWhenModIsNotSim")]
    SCXIDigitizerSimMustNotBeSimWhenModIsNotSim = -201153,
    #[error("SCXIModSimMustMatchSCXIChassisSim")]
    SCXIModSimMustMatchSCXIChassisSim = -201152,
    #[error("SimPXIDevReqSlotAndChassisSpecd")]
    SimPXIDevReqSlotAndChassisSpecd = -201151,
    #[error("SimDevConflictWithRealDev")]
    SimDevConflictWithRealDev = -201150,
    #[error("InsufficientDataForCalibration")]
    InsufficientDataForCalibration = -201149,
    #[error("TriggerChannelMustBeEnabled")]
    TriggerChannelMustBeEnabled = -201148,
    #[error("CalibrationDataConflictCouldNotBeResolved")]
    CalibrationDataConflictCouldNotBeResolved = -201147,
    #[error("SoftwareTooNewForSelfCalibrationData")]
    SoftwareTooNewForSelfCalibrationData = -201146,
    #[error("SoftwareTooNewForExtCalibrationData")]
    SoftwareTooNewForExtCalibrationData = -201145,
    #[error("SelfCalibrationDataTooNewForSoftware")]
    SelfCalibrationDataTooNewForSoftware = -201144,
    #[error("ExtCalibrationDataTooNewForSoftware")]
    ExtCalibrationDataTooNewForSoftware = -201143,
    #[error("SoftwareTooNewForEEPROM")]
    SoftwareTooNewForEEPROM = -201142,
    #[error("EEPROMTooNewForSoftware")]
    EEPROMTooNewForSoftware = -201141,
    #[error("SoftwareTooNewForHardware")]
    SoftwareTooNewForHardware = -201140,
    #[error("HardwareTooNewForSoftware")]
    HardwareTooNewForSoftware = -201139,
    #[error("TaskCannotRestartFirstSampNotAvailToGenerate")]
    TaskCannotRestartFirstSampNotAvailToGenerate = -201138,
    #[error("OnlyUseStartTrigSrcPrptyWithDevDataLines")]
    OnlyUseStartTrigSrcPrptyWithDevDataLines = -201137,
    #[error("OnlyUsePauseTrigSrcPrptyWithDevDataLines")]
    OnlyUsePauseTrigSrcPrptyWithDevDataLines = -201136,
    #[error("OnlyUseRefTrigSrcPrptyWithDevDataLines")]
    OnlyUseRefTrigSrcPrptyWithDevDataLines = -201135,
    #[error("PauseTrigDigPatternSizeDoesNotMatchSrcSize")]
    PauseTrigDigPatternSizeDoesNotMatchSrcSize = -201134,
    #[error("LineConflictCDAQ")]
    LineConflictCDAQ = -201133,
    #[error("CannotWriteBeyondFinalFiniteSample")]
    CannotWriteBeyondFinalFiniteSample = -201132,
    #[error("RefAndStartTriggerSrcCantBeSame")]
    RefAndStartTriggerSrcCantBeSame = -201131,
    #[error("MemMappingIncompatibleWithPhysChansInTask")]
    MemMappingIncompatibleWithPhysChansInTask = -201130,
    #[error("OutputDriveTypeMemMappingConflict")]
    OutputDriveTypeMemMappingConflict = -201129,
    #[error("CAPIDeviceIndexInvalid")]
    CAPIDeviceIndexInvalid = -201128,
    #[error("RatiometricDevicesMustUseExcitationForScaling")]
    RatiometricDevicesMustUseExcitationForScaling = -201127,
    #[error("PropertyRequiresPerDeviceCfg")]
    PropertyRequiresPerDeviceCfg = -201126,
    #[error("AICouplingAndAIInputSourceConflict")]
    AICouplingAndAIInputSourceConflict = -201125,
    #[error("OnlyOneTaskCanPerformDOMemoryMappingAtATime")]
    OnlyOneTaskCanPerformDOMemoryMappingAtATime = -201124,
    #[error("TooManyChansForAnalogRefTrigCDAQ")]
    TooManyChansForAnalogRefTrigCDAQ = -201123,
    #[error("SpecdPropertyValueIsIncompatibleWithSampleTimingType")]
    SpecdPropertyValueIsIncompatibleWithSampleTimingType = -201122,
    #[error("CPUNotSupportedRequireSSE")]
    CPUNotSupportedRequireSSE = -201121,
    #[error("SpecdPropertyValueIsIncompatibleWithSampleTimingResponseMode")]
    SpecdPropertyValueIsIncompatibleWithSampleTimingResponseMode = -201120,
    #[error("ConflictingNextWriteIsLastAndRegenModeProperties")]
    ConflictingNextWriteIsLastAndRegenModeProperties = -201119,
    #[error("MStudioOperationDoesNotSupportDeviceContext")]
    MStudioOperationDoesNotSupportDeviceContext = -201118,
    #[error("PropertyValueInChannelExpansionContextInvalid")]
    PropertyValueInChannelExpansionContextInvalid = -201117,
    #[error("HWTimedNonBufferedAONotSupported")]
    HWTimedNonBufferedAONotSupported = -201116,
    #[error("WaveformLengthNotMultOfQuantum")]
    WaveformLengthNotMultOfQuantum = -201115,
    #[error("DSAExpansionMixedBoardsWrongOrderInPXIChassis")]
    DSAExpansionMixedBoardsWrongOrderInPXIChassis = -201114,
    #[error("PowerLevelTooLowForOOK")]
    PowerLevelTooLowForOOK = -201113,
    #[error("DeviceComponentTestFailure")]
    DeviceComponentTestFailure = -201112,
    #[error("UserDefinedWfmWithOOKUnsupported")]
    UserDefinedWfmWithOOKUnsupported = -201111,
    #[error("InvalidDigitalModulationUserDefinedWaveform")]
    InvalidDigitalModulationUserDefinedWaveform = -201110,
    #[error("BothRefInAndRefOutEnabled")]
    BothRefInAndRefOutEnabled = -201109,
    #[error("BothAnalogAndDigitalModulationEnabled")]
    BothAnalogAndDigitalModulationEnabled = -201108,
    #[error("BufferedOpsNotSupportedInSpecdSlotForCDAQ")]
    BufferedOpsNotSupportedInSpecdSlotForCDAQ = -201107,
    #[error("PhysChanNotSupportedInSpecdSlotForCDAQ")]
    PhysChanNotSupportedInSpecdSlotForCDAQ = -201106,
    #[error("ResourceReservedWithConflictingSettings")]
    ResourceReservedWithConflictingSettings = -201105,
    #[error("InconsistentAnalogTrigSettingsCDAQ")]
    InconsistentAnalogTrigSettingsCDAQ = -201104,
    #[error("TooManyChansForAnalogPauseTrigCDAQ")]
    TooManyChansForAnalogPauseTrigCDAQ = -201103,
    #[error("AnalogTrigNotFirstInScanListCDAQ")]
    AnalogTrigNotFirstInScanListCDAQ = -201102,
    #[error("TooManyChansGivenTimingType")]
    TooManyChansGivenTimingType = -201101,
    #[error("SampClkTimebaseDivWithExtSampClk")]
    SampClkTimebaseDivWithExtSampClk = -201100,
    #[error("CantSaveTaskWithPerDeviceTimingProperties")]
    CantSaveTaskWithPerDeviceTimingProperties = -201099,
    #[error("ConflictingAutoZeroMode")]
    ConflictingAutoZeroMode = -201098,
    #[error("SampClkRateNotSupportedWithEAREnabled")]
    SampClkRateNotSupportedWithEAREnabled = -201097,
    #[error("SampClkTimebaseRateNotSpecd")]
    SampClkTimebaseRateNotSpecd = -201096,
    #[error("SessionCorruptedByDLLReload")]
    SessionCorruptedByDLLReload = -201095,
    #[error("ActiveDevNotSupportedWithChanExpansion")]
    ActiveDevNotSupportedWithChanExpansion = -201094,
    #[error("SampClkRateInvalid")]
    SampClkRateInvalid = -201093,
    #[error("ExtSyncPulseSrcCannotBeExported")]
    ExtSyncPulseSrcCannotBeExported = -201092,
    #[error("SyncPulseMinDelayToStartNeededForExtSyncPulseSrc")]
    SyncPulseMinDelayToStartNeededForExtSyncPulseSrc = -201091,
    #[error("SyncPulseSrcInvalid")]
    SyncPulseSrcInvalid = -201090,
    #[error("SampClkTimebaseRateInvalid")]
    SampClkTimebaseRateInvalid = -201089,
    #[error("SampClkTimebaseSrcInvalid")]
    SampClkTimebaseSrcInvalid = -201088,
    #[error("SampClkRateMustBeSpecd")]
    SampClkRateMustBeSpecd = -201087,
    #[error("InvalidAttributeName")]
    InvalidAttributeName = -201086,
    #[error("CJCChanNameMustBeSetWhenCJCSrcIsScannableChan")]
    CJCChanNameMustBeSetWhenCJCSrcIsScannableChan = -201085,
    #[error("HiddenChanMissingInChansPropertyInCfgFile")]
    HiddenChanMissingInChansPropertyInCfgFile = -201084,
    #[error("ChanNamesNotSpecdInCfgFile")]
    ChanNamesNotSpecdInCfgFile = -201083,
    #[error("DuplicateHiddenChanNamesInCfgFile")]
    DuplicateHiddenChanNamesInCfgFile = -201082,
    #[error("DuplicateChanNameInCfgFile")]
    DuplicateChanNameInCfgFile = -201081,
    #[error("InvalidSCCModuleForSlotSpecd")]
    InvalidSCCModuleForSlotSpecd = -201080,
    #[error("InvalidSCCSlotNumberSpecd")]
    InvalidSCCSlotNumberSpecd = -201079,
    #[error("InvalidSectionIdentifier")]
    InvalidSectionIdentifier = -201078,
    #[error("InvalidSectionName")]
    InvalidSectionName = -201077,
    #[error("DAQmxVersionNotSupported")]
    DAQmxVersionNotSupported = -201076,
    #[error("SWObjectsFoundInFile")]
    SWObjectsFoundInFile = -201075,
    #[error("HWObjectsFoundInFile")]
    HWObjectsFoundInFile = -201074,
    #[error("LocalChannelSpecdWithNoParentTask")]
    LocalChannelSpecdWithNoParentTask = -201073,
    #[error("TaskReferencesMissingLocalChannel")]
    TaskReferencesMissingLocalChannel = -201072,
    #[error("TaskReferencesLocalChannelFromOtherTask")]
    TaskReferencesLocalChannelFromOtherTask = -201071,
    #[error("TaskMissingChannelProperty")]
    TaskMissingChannelProperty = -201070,
    #[error("InvalidLocalChanName")]
    InvalidLocalChanName = -201069,
    #[error("InvalidEscapeCharacterInString")]
    InvalidEscapeCharacterInString = -201068,
    #[error("InvalidTableIdentifier")]
    InvalidTableIdentifier = -201067,
    #[error("ValueFoundInInvalidColumn")]
    ValueFoundInInvalidColumn = -201066,
    #[error("MissingStartOfTable")]
    MissingStartOfTable = -201065,
    #[error("FileMissingRequiredDAQmxHeader")]
    FileMissingRequiredDAQmxHeader = -201064,
    #[error("DeviceIDDoesNotMatch")]
    DeviceIDDoesNotMatch = -201063,
    #[error("BufferedOperationsNotSupportedOnSelectedLines")]
    BufferedOperationsNotSupportedOnSelectedLines = -201062,
    #[error("PropertyConflictsWithScale")]
    PropertyConflictsWithScale = -201061,
    #[error("InvalidINIFileSyntax")]
    InvalidINIFileSyntax = -201060,
    #[error("DeviceInfoFailedPXIChassisNotIdentified")]
    DeviceInfoFailedPXIChassisNotIdentified = -201059,
    #[error("InvalidHWProductNumber")]
    InvalidHWProductNumber = -201058,
    #[error("InvalidHWProductType")]
    InvalidHWProductType = -201057,
    #[error("InvalidNumericFormatSpecd")]
    InvalidNumericFormatSpecd = -201056,
    #[error("DuplicatePropertyInObject")]
    DuplicatePropertyInObject = -201055,
    #[error("InvalidEnumValueSpecd")]
    InvalidEnumValueSpecd = -201054,
    #[error("TEDSSensorPhysicalChannelConflict")]
    TEDSSensorPhysicalChannelConflict = -201053,
    #[error("TooManyPhysicalChansForTEDSInterfaceSpecd")]
    TooManyPhysicalChansForTEDSInterfaceSpecd = -201052,
    #[error("IncapableTEDSInterfaceControllingDeviceSpecd")]
    IncapableTEDSInterfaceControllingDeviceSpecd = -201051,
    #[error("SCCCarrierSpecdIsMissing")]
    SCCCarrierSpecdIsMissing = -201050,
    #[error("IncapableSCCDigitizingDeviceSpecd")]
    IncapableSCCDigitizingDeviceSpecd = -201049,
    #[error("AccessorySettingNotApplicable")]
    AccessorySettingNotApplicable = -201048,
    #[error("DeviceAndConnectorSpecdAlreadyOccupied")]
    DeviceAndConnectorSpecdAlreadyOccupied = -201047,
    #[error("IllegalAccessoryTypeForDeviceSpecd")]
    IllegalAccessoryTypeForDeviceSpecd = -201046,
    #[error("InvalidDeviceConnectorNumberSpecd")]
    InvalidDeviceConnectorNumberSpecd = -201045,
    #[error("InvalidAccessoryName")]
    InvalidAccessoryName = -201044,
    #[error("MoreThanOneMatchForSpecdDevice")]
    MoreThanOneMatchForSpecdDevice = -201043,
    #[error("NoMatchForSpecdDevice")]
    NoMatchForSpecdDevice = -201042,
    #[error("ProductTypeAndProductNumberConflict")]
    ProductTypeAndProductNumberConflict = -201041,
    #[error("ExtraPropertyDetectedInSpecdObject")]
    ExtraPropertyDetectedInSpecdObject = -201040,
    #[error("RequiredPropertyMissing")]
    RequiredPropertyMissing = -201039,
    #[error("CantSetAuthorForLocalChan")]
    CantSetAuthorForLocalChan = -201038,
    #[error("InvalidTimeValue")]
    InvalidTimeValue = -201037,
    #[error("InvalidTimeFormat")]
    InvalidTimeFormat = -201036,
    #[error("DigDevChansSpecdInModeOtherThanParallel")]
    DigDevChansSpecdInModeOtherThanParallel = -201035,
    #[error("CascadeDigitizationModeNotSupported")]
    CascadeDigitizationModeNotSupported = -201034,
    #[error("SpecdSlotAlreadyOccupied")]
    SpecdSlotAlreadyOccupied = -201033,
    #[error("InvalidSCXISlotNumberSpecd")]
    InvalidSCXISlotNumberSpecd = -201032,
    #[error("AddressAlreadyInUse")]
    AddressAlreadyInUse = -201031,
    #[error("SpecdDeviceDoesNotSupportRTSI")]
    SpecdDeviceDoesNotSupportRTSI = -201030,
    #[error("SpecdDeviceIsAlreadyOnRTSIBus")]
    SpecdDeviceIsAlreadyOnRTSIBus = -201029,
    #[error("IdentifierInUse")]
    IdentifierInUse = -201028,
    #[error("WaitForNextSampleClockOrReadDetected3OrMoreMissedSampClks")]
    WaitForNextSampleClockOrReadDetected3OrMoreMissedSampClks = -201027,
    #[error("HWTimedAndDataXferPIO")]
    HWTimedAndDataXferPIO = -201026,
    #[error("NonBufferedAndHWTimed")]
    NonBufferedAndHWTimed = -201025,
    #[error("CTROutSampClkPeriodShorterThanGenPulseTrainPeriodPolled")]
    CTROutSampClkPeriodShorterThanGenPulseTrainPeriodPolled = -201024,
    #[error("CTROutSampClkPeriodShorterThanGenPulseTrainPeriod2")]
    CTROutSampClkPeriodShorterThanGenPulseTrainPeriod2 = -201023,
    #[error("COCannotKeepUpInHWTimedSinglePointPolled")]
    COCannotKeepUpInHWTimedSinglePointPolled = -201022,
    #[error("WriteRecoveryCannotKeepUpInHWTimedSinglePoint")]
    WriteRecoveryCannotKeepUpInHWTimedSinglePoint = -201021,
    #[error("NoChangeDetectionOnSelectedLineForDevice")]
    NoChangeDetectionOnSelectedLineForDevice = -201020,
    #[error("SMIOPauseTriggersNotSupportedWithChannelExpansion")]
    SMIOPauseTriggersNotSupportedWithChannelExpansion = -201019,
    #[error("ClockMasterForExternalClockNotLongestPipeline")]
    ClockMasterForExternalClockNotLongestPipeline = -201018,
    #[error("UnsupportedUnicodeByteOrderMarker")]
    UnsupportedUnicodeByteOrderMarker = -201017,
    #[error("TooManyInstructionsInLoopInScript")]
    TooManyInstructionsInLoopInScript = -201016,
    #[error("PLLNotLocked")]
    PLLNotLocked = -201015,
    #[error("IfElseBlockNotAllowedInFiniteRepeatLoopInScript")]
    IfElseBlockNotAllowedInFiniteRepeatLoopInScript = -201014,
    #[error("IfElseBlockNotAllowedInConditionalRepeatLoopInScript")]
    IfElseBlockNotAllowedInConditionalRepeatLoopInScript = -201013,
    #[error("ClearIsLastInstructionInIfElseBlockInScript")]
    ClearIsLastInstructionInIfElseBlockInScript = -201012,
    #[error("InvalidWaitDurationBeforeIfElseBlockInScript")]
    InvalidWaitDurationBeforeIfElseBlockInScript = -201011,
    #[error("MarkerPosInvalidBeforeIfElseBlockInScript")]
    MarkerPosInvalidBeforeIfElseBlockInScript = -201010,
    #[error("InvalidSubsetLengthBeforeIfElseBlockInScript")]
    InvalidSubsetLengthBeforeIfElseBlockInScript = -201009,
    #[error("InvalidWaveformLengthBeforeIfElseBlockInScript")]
    InvalidWaveformLengthBeforeIfElseBlockInScript = -201008,
    #[error("GenerateOrFiniteWaitInstructionExpectedBeforeIfElseBlockInScript")]
    GenerateOrFiniteWaitInstructionExpectedBeforeIfElseBlockInScript = -201007,
    #[error("CalPasswordNotSupported")]
    CalPasswordNotSupported = -201006,
    #[error("SetupCalNeededBeforeAdjustCal")]
    SetupCalNeededBeforeAdjustCal = -201005,
    #[error("MultipleChansNotSupportedDuringCalSetup")]
    MultipleChansNotSupportedDuringCalSetup = -201004,
    #[error("DevCannotBeAccessed")]
    DevCannotBeAccessed = -201003,
    #[error("SampClkRateDoesntMatchSampClkSrc")]
    SampClkRateDoesntMatchSampClkSrc = -201002,
    #[error("SampClkRateNotSupportedWithEARDisabled")]
    SampClkRateNotSupportedWithEARDisabled = -201001,
    #[error("LabVIEWVersionDoesntSupportDAQmxEvents")]
    LabVIEWVersionDoesntSupportDAQmxEvents = -201000,
    #[error("COReadyForNewValNotSupportedWithOnDemand")]
    COReadyForNewValNotSupportedWithOnDemand = -200999,
    #[error("CIHWTimedSinglePointNotSupportedForMeasType")]
    CIHWTimedSinglePointNotSupportedForMeasType = -200998,
    #[error("OnDemandNotSupportedWithHWTimedSinglePoint")]
    OnDemandNotSupportedWithHWTimedSinglePoint = -200997,
    #[error("HWTimedSinglePointAndDataXferNotProgIO")]
    HWTimedSinglePointAndDataXferNotProgIO = -200996,
    #[error("MemMapAndHWTimedSinglePoint")]
    MemMapAndHWTimedSinglePoint = -200995,
    #[error("CannotSetPropertyWhenHWTimedSinglePointTaskIsRunning")]
    CannotSetPropertyWhenHWTimedSinglePointTaskIsRunning = -200994,
    #[error("CTROutSampClkPeriodShorterThanGenPulseTrainPeriod")]
    CTROutSampClkPeriodShorterThanGenPulseTrainPeriod = -200993,
    #[error("TooManyEventsGenerated")]
    TooManyEventsGenerated = -200992,
    #[error("MStudioCppRemoveEventsBeforeStop")]
    MStudioCppRemoveEventsBeforeStop = -200991,
    #[error("CAPICannotRegisterSyncEventsFromMultipleThreads")]
    CAPICannotRegisterSyncEventsFromMultipleThreads = -200990,
    #[error("ReadWaitNextSampClkWaitMismatchTwo")]
    ReadWaitNextSampClkWaitMismatchTwo = -200989,
    #[error("ReadWaitNextSampClkWaitMismatchOne")]
    ReadWaitNextSampClkWaitMismatchOne = -200988,
    #[error("DAQmxSignalEventTypeNotSupportedByChanTypesOrDevicesInTask")]
    DAQmxSignalEventTypeNotSupportedByChanTypesOrDevicesInTask = -200987,
    #[error("CannotUnregisterDAQmxSoftwareEventWhileTaskIsRunning")]
    CannotUnregisterDAQmxSoftwareEventWhileTaskIsRunning = -200986,
    #[error("AutoStartWriteNotAllowedEventRegistered")]
    AutoStartWriteNotAllowedEventRegistered = -200985,
    #[error("AutoStartReadNotAllowedEventRegistered")]
    AutoStartReadNotAllowedEventRegistered = -200984,
    #[error("CannotGetPropertyWhenTaskNotReservedCommittedOrRunning")]
    CannotGetPropertyWhenTaskNotReservedCommittedOrRunning = -200983,
    #[error("SignalEventsNotSupportedByDevice")]
    SignalEventsNotSupportedByDevice = -200982,
    #[error("EveryNSamplesAcqIntoBufferEventNotSupportedByDevice")]
    EveryNSamplesAcqIntoBufferEventNotSupportedByDevice = -200981,
    #[error("EveryNSampsTransferredFromBufferEventNotSupportedByDevice")]
    EveryNSampsTransferredFromBufferEventNotSupportedByDevice = -200980,
    #[error("CAPISyncEventsTaskStateChangeNotAllowedFromDifferentThread")]
    CAPISyncEventsTaskStateChangeNotAllowedFromDifferentThread = -200979,
    #[error("DAQmxSWEventsWithDifferentCallMechanisms")]
    DAQmxSWEventsWithDifferentCallMechanisms = -200978,
    #[error("CantSaveChanWithPolyCalScaleAndAllowInteractiveEdit")]
    CantSaveChanWithPolyCalScaleAndAllowInteractiveEdit = -200977,
    #[error("ChanDoesNotSupportCJC")]
    ChanDoesNotSupportCJC = -200976,
    #[error("COReadyForNewValNotSupportedWithHWTimedSinglePoint")]
    COReadyForNewValNotSupportedWithHWTimedSinglePoint = -200975,
    #[error("DACAllowConnToGndNotSupportedByDevWhenRefSrcExt")]
    DACAllowConnToGndNotSupportedByDevWhenRefSrcExt = -200974,
    #[error("CantGetPropertyTaskNotRunning")]
    CantGetPropertyTaskNotRunning = -200973,
    #[error("CantSetPropertyTaskNotRunning")]
    CantSetPropertyTaskNotRunning = -200972,
    #[error("CantSetPropertyTaskNotRunningCommitted")]
    CantSetPropertyTaskNotRunningCommitted = -200971,
    #[error("AIEveryNSampsEventIntervalNotMultipleOf2")]
    AIEveryNSampsEventIntervalNotMultipleOf2 = -200970,
    #[error("InvalidTEDSPhysChanNotAI")]
    InvalidTEDSPhysChanNotAI = -200969,
    #[error("CAPICannotPerformTaskOperationInAsyncCallback")]
    CAPICannotPerformTaskOperationInAsyncCallback = -200968,
    #[error("EveryNSampsTransferredFromBufferEventAlreadyRegistered")]
    EveryNSampsTransferredFromBufferEventAlreadyRegistered = -200967,
    #[error("EveryNSampsAcqIntoBufferEventAlreadyRegistered")]
    EveryNSampsAcqIntoBufferEventAlreadyRegistered = -200966,
    #[error("EveryNSampsTransferredFromBufferNotForInput")]
    EveryNSampsTransferredFromBufferNotForInput = -200965,
    #[error("EveryNSampsAcqIntoBufferNotForOutput")]
    EveryNSampsAcqIntoBufferNotForOutput = -200964,
    #[error("AOSampTimingTypeDifferentIn2Tasks")]
    AOSampTimingTypeDifferentIn2Tasks = -200963,
    #[error("CouldNotDownloadFirmwareHWDamaged")]
    CouldNotDownloadFirmwareHWDamaged = -200962,
    #[error("CouldNotDownloadFirmwareFileMissingOrDamaged")]
    CouldNotDownloadFirmwareFileMissingOrDamaged = -200961,
    #[error("CannotRegisterDAQmxSoftwareEventWhileTaskIsRunning")]
    CannotRegisterDAQmxSoftwareEventWhileTaskIsRunning = -200960,
    #[error("DifferentRawDataCompression")]
    DifferentRawDataCompression = -200959,
    #[error("ConfiguredTEDSInterfaceDevNotDetected")]
    ConfiguredTEDSInterfaceDevNotDetected = -200958,
    #[error("CompressedSampSizeExceedsResolution")]
    CompressedSampSizeExceedsResolution = -200957,
    #[error("ChanDoesNotSupportCompression")]
    ChanDoesNotSupportCompression = -200956,
    #[error("DifferentRawDataFormats")]
    DifferentRawDataFormats = -200955,
    #[error("SampClkOutputTermIncludesStartTrigSrc")]
    SampClkOutputTermIncludesStartTrigSrc = -200954,
    #[error("StartTrigSrcEqualToSampClkSrc")]
    StartTrigSrcEqualToSampClkSrc = -200953,
    #[error("EventOutputTermIncludesTrigSrc")]
    EventOutputTermIncludesTrigSrc = -200952,
    #[error("COMultipleWritesBetweenSampClks")]
    COMultipleWritesBetweenSampClks = -200951,
    #[error("DoneEventAlreadyRegistered")]
    DoneEventAlreadyRegistered = -200950,
    #[error("SignalEventAlreadyRegistered")]
    SignalEventAlreadyRegistered = -200949,
    #[error("CannotHaveTimedLoopAndDAQmxSignalEventsInSameTask")]
    CannotHaveTimedLoopAndDAQmxSignalEventsInSameTask = -200948,
    #[error("NeedLabVIEW711PatchToUseDAQmxEvents")]
    NeedLabVIEW711PatchToUseDAQmxEvents = -200947,
    #[error("StartFailedDueToWriteFailure")]
    StartFailedDueToWriteFailure = -200946,
    #[error("DataXferCustomThresholdNotDMAXferMethodSpecifiedForDev")]
    DataXferCustomThresholdNotDMAXferMethodSpecifiedForDev = -200945,
    #[error("DataXferRequestConditionNotSpecifiedForCustomThreshold")]
    DataXferRequestConditionNotSpecifiedForCustomThreshold = -200944,
    #[error("DataXferCustomThresholdNotSpecified")]
    DataXferCustomThresholdNotSpecified = -200943,
    #[error("CAPISyncCallbackNotSupportedOnThisPlatform")]
    CAPISyncCallbackNotSupportedOnThisPlatform = -200942,
    #[error("CalChanReversePolyCoefNotSpecd")]
    CalChanReversePolyCoefNotSpecd = -200941,
    #[error("CalChanForwardPolyCoefNotSpecd")]
    CalChanForwardPolyCoefNotSpecd = -200940,
    #[error("ChanCalRepeatedNumberInPreScaledVals")]
    ChanCalRepeatedNumberInPreScaledVals = -200939,
    #[error("ChanCalTableNumScaledNotEqualNumPrescaledVals")]
    ChanCalTableNumScaledNotEqualNumPrescaledVals = -200938,
    #[error("ChanCalTableScaledValsNotSpecd")]
    ChanCalTableScaledValsNotSpecd = -200937,
    #[error("ChanCalTablePreScaledValsNotSpecd")]
    ChanCalTablePreScaledValsNotSpecd = -200936,
    #[error("ChanCalScaleTypeNotSet")]
    ChanCalScaleTypeNotSet = -200935,
    #[error("ChanCalExpired")]
    ChanCalExpired = -200934,
    #[error("ChanCalExpirationDateNotSet")]
    ChanCalExpirationDateNotSet = -200933,
    #[error("ThreeOutputPortCombinationGivenSampTimingType653x")]
    ThreeOutputPortCombinationGivenSampTimingType653x = -200932,
    #[error("ThreeInputPortCombinationGivenSampTimingType653x")]
    ThreeInputPortCombinationGivenSampTimingType653x = -200931,
    #[error("TwoOutputPortCombinationGivenSampTimingType653x")]
    TwoOutputPortCombinationGivenSampTimingType653x = -200930,
    #[error("TwoInputPortCombinationGivenSampTimingType653x")]
    TwoInputPortCombinationGivenSampTimingType653x = -200929,
    #[error("PatternMatcherMayBeUsedByOneTrigOnly")]
    PatternMatcherMayBeUsedByOneTrigOnly = -200928,
    #[error("NoChansSpecdForPatternSource")]
    NoChansSpecdForPatternSource = -200927,
    #[error("ChangeDetectionChanNotInTask")]
    ChangeDetectionChanNotInTask = -200926,
    #[error("ChangeDetectionChanNotTristated")]
    ChangeDetectionChanNotTristated = -200925,
    #[error("WaitModeValueNotSupportedNonBuffered")]
    WaitModeValueNotSupportedNonBuffered = -200924,
    #[error("WaitModePropertyNotSupportedNonBuffered")]
    WaitModePropertyNotSupportedNonBuffered = -200923,
    #[error("CantSavePerLineConfigDigChanSoInteractiveEditsAllowed")]
    CantSavePerLineConfigDigChanSoInteractiveEditsAllowed = -200922,
    #[error("CantSaveNonPortMultiLineDigChanSoInteractiveEditsAllowed")]
    CantSaveNonPortMultiLineDigChanSoInteractiveEditsAllowed = -200921,
    #[error("BufferSizeNotMultipleOfEveryNSampsEventIntervalNoIrqOnDev")]
    BufferSizeNotMultipleOfEveryNSampsEventIntervalNoIrqOnDev = -200920,
    #[error("GlobalTaskNameAlreadyChanName")]
    GlobalTaskNameAlreadyChanName = -200919,
    #[error("GlobalChanNameAlreadyTaskName")]
    GlobalChanNameAlreadyTaskName = -200918,
    #[error("AOEveryNSampsEventIntervalNotMultipleOf2")]
    AOEveryNSampsEventIntervalNotMultipleOf2 = -200917,
    #[error("SampleTimebaseDivisorNotSupportedGivenTimingType")]
    SampleTimebaseDivisorNotSupportedGivenTimingType = -200916,
    #[error("HandshakeEventOutputTermNotSupportedGivenTimingType")]
    HandshakeEventOutputTermNotSupportedGivenTimingType = -200915,
    #[error("ChangeDetectionOutputTermNotSupportedGivenTimingType")]
    ChangeDetectionOutputTermNotSupportedGivenTimingType = -200914,
    #[error("ReadyForTransferOutputTermNotSupportedGivenTimingType")]
    ReadyForTransferOutputTermNotSupportedGivenTimingType = -200913,
    #[error("RefTrigOutputTermNotSupportedGivenTimingType")]
    RefTrigOutputTermNotSupportedGivenTimingType = -200912,
    #[error("StartTrigOutputTermNotSupportedGivenTimingType")]
    StartTrigOutputTermNotSupportedGivenTimingType = -200911,
    #[error("SampClockOutputTermNotSupportedGivenTimingType")]
    SampClockOutputTermNotSupportedGivenTimingType = -200910,
    #[error("TwentyMhzTimebaseNotSupportedGivenTimingType")]
    TwentyMhzTimebaseNotSupportedGivenTimingType = -200909,
    #[error("SampClockSourceNotSupportedGivenTimingType")]
    SampClockSourceNotSupportedGivenTimingType = -200908,
    #[error("RefTrigTypeNotSupportedGivenTimingType")]
    RefTrigTypeNotSupportedGivenTimingType = -200907,
    #[error("PauseTrigTypeNotSupportedGivenTimingType")]
    PauseTrigTypeNotSupportedGivenTimingType = -200906,
    #[error("HandshakeTrigTypeNotSupportedGivenTimingType")]
    HandshakeTrigTypeNotSupportedGivenTimingType = -200905,
    #[error("StartTrigTypeNotSupportedGivenTimingType")]
    StartTrigTypeNotSupportedGivenTimingType = -200904,
    #[error("RefClkSrcNotSupported")]
    RefClkSrcNotSupported = -200903,
    #[error("DataVoltageLowAndHighIncompatible")]
    DataVoltageLowAndHighIncompatible = -200902,
    #[error("InvalidCharInDigPatternString")]
    InvalidCharInDigPatternString = -200901,
    #[error("CantUsePort3AloneGivenSampTimingTypeOn653x")]
    CantUsePort3AloneGivenSampTimingTypeOn653x = -200900,
    #[error("CantUsePort1AloneGivenSampTimingTypeOn653x")]
    CantUsePort1AloneGivenSampTimingTypeOn653x = -200899,
    #[error("PartialUseOfPhysicalLinesWithinPortNotSupported653x")]
    PartialUseOfPhysicalLinesWithinPortNotSupported653x = -200898,
    #[error("PhysicalChanNotSupportedGivenSampTimingType653x")]
    PhysicalChanNotSupportedGivenSampTimingType653x = -200897,
    #[error("CanExportOnlyDigEdgeTrigs")]
    CanExportOnlyDigEdgeTrigs = -200896,
    #[error("RefTrigDigPatternSizeDoesNotMatchSourceSize")]
    RefTrigDigPatternSizeDoesNotMatchSourceSize = -200895,
    #[error("StartTrigDigPatternSizeDoesNotMatchSourceSize")]
    StartTrigDigPatternSizeDoesNotMatchSourceSize = -200894,
    #[error("ChangeDetectionRisingAndFallingEdgeChanDontMatch")]
    ChangeDetectionRisingAndFallingEdgeChanDontMatch = -200893,
    #[error("PhysicalChansForChangeDetectionAndPatternMatch653x")]
    PhysicalChansForChangeDetectionAndPatternMatch653x = -200892,
    #[error("CanExportOnlyOnboardSampClk")]
    CanExportOnlyOnboardSampClk = -200891,
    #[error("InternalSampClkNotRisingEdge")]
    InternalSampClkNotRisingEdge = -200890,
    #[error("RefTrigDigPatternChanNotInTask")]
    RefTrigDigPatternChanNotInTask = -200889,
    #[error("RefTrigDigPatternChanNotTristated")]
    RefTrigDigPatternChanNotTristated = -200888,
    #[error("StartTrigDigPatternChanNotInTask")]
    StartTrigDigPatternChanNotInTask = -200887,
    #[error("StartTrigDigPatternChanNotTristated")]
    StartTrigDigPatternChanNotTristated = -200886,
    #[error("PXIStarAndClock10Sync")]
    PXIStarAndClock10Sync = -200885,
    #[error("GlobalChanCannotBeSavedSoInteractiveEditsAllowed")]
    GlobalChanCannotBeSavedSoInteractiveEditsAllowed = -200884,
    #[error("TaskCannotBeSavedSoInteractiveEditsAllowed")]
    TaskCannotBeSavedSoInteractiveEditsAllowed = -200883,
    #[error("InvalidGlobalChan")]
    InvalidGlobalChan = -200882,
    #[error("EveryNSampsEventAlreadyRegistered")]
    EveryNSampsEventAlreadyRegistered = -200881,
    #[error("EveryNSampsEventIntervalZeroNotSupported")]
    EveryNSampsEventIntervalZeroNotSupported = -200880,
    #[error("ChanSizeTooBigForU16PortWrite")]
    ChanSizeTooBigForU16PortWrite = -200879,
    #[error("ChanSizeTooBigForU16PortRead")]
    ChanSizeTooBigForU16PortRead = -200878,
    #[error("BufferSizeNotMultipleOfEveryNSampsEventIntervalWhenDMA")]
    BufferSizeNotMultipleOfEveryNSampsEventIntervalWhenDMA = -200877,
    #[error("WriteWhenTaskNotRunningCOTicks")]
    WriteWhenTaskNotRunningCOTicks = -200876,
    #[error("WriteWhenTaskNotRunningCOFreq")]
    WriteWhenTaskNotRunningCOFreq = -200875,
    #[error("WriteWhenTaskNotRunningCOTime")]
    WriteWhenTaskNotRunningCOTime = -200874,
    #[error("AOMinMaxNotSupportedDACRangeTooSmall")]
    AOMinMaxNotSupportedDACRangeTooSmall = -200873,
    #[error("AOMinMaxNotSupportedGivenDACRange")]
    AOMinMaxNotSupportedGivenDACRange = -200872,
    #[error("AOMinMaxNotSupportedGivenDACRangeAndOffsetVal")]
    AOMinMaxNotSupportedGivenDACRangeAndOffsetVal = -200871,
    #[error("AOMinMaxNotSupportedDACOffsetValInappropriate")]
    AOMinMaxNotSupportedDACOffsetValInappropriate = -200870,
    #[error("AOMinMaxNotSupportedGivenDACOffsetVal")]
    AOMinMaxNotSupportedGivenDACOffsetVal = -200869,
    #[error("AOMinMaxNotSupportedDACRefValTooSmall")]
    AOMinMaxNotSupportedDACRefValTooSmall = -200868,
    #[error("AOMinMaxNotSupportedGivenDACRefVal")]
    AOMinMaxNotSupportedGivenDACRefVal = -200867,
    #[error("AOMinMaxNotSupportedGivenDACRefAndOffsetVal")]
    AOMinMaxNotSupportedGivenDACRefAndOffsetVal = -200866,
    #[error("WhenAcqCompAndNumSampsPerChanExceedsOnBrdBufSize")]
    WhenAcqCompAndNumSampsPerChanExceedsOnBrdBufSize = -200865,
    #[error("WhenAcqCompAndNoRefTrig")]
    WhenAcqCompAndNoRefTrig = -200864,
    #[error("WaitForNextSampClkNotSupported")]
    WaitForNextSampClkNotSupported = -200863,
    #[error("DevInUnidentifiedPXIChassis")]
    DevInUnidentifiedPXIChassis = -200862,
    #[error("MaxSoundPressureMicSensitivitRelatedAIPropertiesNotSupportedByDev")]
    MaxSoundPressureMicSensitivitRelatedAIPropertiesNotSupportedByDev = -200861,
    #[error("MaxSoundPressureAndMicSensitivityNotSupportedByDev")]
    MaxSoundPressureAndMicSensitivityNotSupportedByDev = -200860,
    #[error("AOBufferSizeZeroForSampClkTimingType")]
    AOBufferSizeZeroForSampClkTimingType = -200859,
    #[error("AOCallWriteBeforeStartForSampClkTimingType")]
    AOCallWriteBeforeStartForSampClkTimingType = -200858,
    #[error("InvalidCalLowPassCutoffFreq")]
    InvalidCalLowPassCutoffFreq = -200857,
    #[error("SimulationCannotBeDisabledForDevCreatedAsSimulatedDev")]
    SimulationCannotBeDisabledForDevCreatedAsSimulatedDev = -200856,
    #[error("CannotAddNewDevsAfterTaskConfiguration")]
    CannotAddNewDevsAfterTaskConfiguration = -200855,
    #[error("DifftSyncPulseSrcAndSampClkTimebaseSrcDevMultiDevTask")]
    DifftSyncPulseSrcAndSampClkTimebaseSrcDevMultiDevTask = -200854,
    #[error("TermWithoutDevInMultiDevTask")]
    TermWithoutDevInMultiDevTask = -200853,
    #[error("SyncNoDevSampClkTimebaseOrSyncPulseInPXISlot2")]
    SyncNoDevSampClkTimebaseOrSyncPulseInPXISlot2 = -200852,
    #[error("PhysicalChanNotOnThisConnector")]
    PhysicalChanNotOnThisConnector = -200851,
    #[error("NumSampsToWaitNotGreaterThanZeroInScript")]
    NumSampsToWaitNotGreaterThanZeroInScript = -200850,
    #[error("NumSampsToWaitNotMultipleOfAlignmentQuantumInScript")]
    NumSampsToWaitNotMultipleOfAlignmentQuantumInScript = -200849,
    #[error("EveryNSamplesEventNotSupportedForNonBufferedTasks")]
    EveryNSamplesEventNotSupportedForNonBufferedTasks = -200848,
    #[error("BufferedAndDataXferPIO")]
    BufferedAndDataXferPIO = -200847,
    #[error("CannotWriteWhenAutoStartFalseAndTaskNotRunning")]
    CannotWriteWhenAutoStartFalseAndTaskNotRunning = -200846,
    #[error("NonBufferedAndDataXferInterrupts")]
    NonBufferedAndDataXferInterrupts = -200845,
    #[error("WriteFailedMultipleCtrsWithFREQOUT")]
    WriteFailedMultipleCtrsWithFREQOUT = -200844,
    #[error("ReadNotCompleteBefore3SampClkEdges")]
    ReadNotCompleteBefore3SampClkEdges = -200843,
    #[error("CtrHWTimedSinglePointAndDataXferNotProgIO")]
    CtrHWTimedSinglePointAndDataXferNotProgIO = -200842,
    #[error("PrescalerNot1ForInputTerminal")]
    PrescalerNot1ForInputTerminal = -200841,
    #[error("PrescalerNot1ForTimebaseSrc")]
    PrescalerNot1ForTimebaseSrc = -200840,
    #[error("SampClkTimingTypeWhenTristateIsFalse")]
    SampClkTimingTypeWhenTristateIsFalse = -200839,
    #[error("OutputBufferSizeNotMultOfXferSize")]
    OutputBufferSizeNotMultOfXferSize = -200838,
    #[error("SampPerChanNotMultOfXferSize")]
    SampPerChanNotMultOfXferSize = -200837,
    #[error("WriteToTEDSFailed")]
    WriteToTEDSFailed = -200836,
    #[error("SCXIDevNotUsablePowerTurnedOff")]
    SCXIDevNotUsablePowerTurnedOff = -200835,
    #[error("CannotReadWhenAutoStartFalseBufSizeZeroAndTaskNotRunning")]
    CannotReadWhenAutoStartFalseBufSizeZeroAndTaskNotRunning = -200834,
    #[error("CannotReadWhenAutoStartFalseHWTimedSinglePtAndTaskNotRunning")]
    CannotReadWhenAutoStartFalseHWTimedSinglePtAndTaskNotRunning = -200833,
    #[error("CannotReadWhenAutoStartFalseOnDemandAndTaskNotRunning")]
    CannotReadWhenAutoStartFalseOnDemandAndTaskNotRunning = -200832,
    #[error("SimultaneousAOWhenNotOnDemandTiming")]
    SimultaneousAOWhenNotOnDemandTiming = -200831,
    #[error("MemMapAndSimultaneousAO")]
    MemMapAndSimultaneousAO = -200830,
    #[error("WriteFailedMultipleCOOutputTypes")]
    WriteFailedMultipleCOOutputTypes = -200829,
    #[error("WriteToTEDSNotSupportedOnRT")]
    WriteToTEDSNotSupportedOnRT = -200828,
    #[error("VirtualTEDSDataFileError")]
    VirtualTEDSDataFileError = -200827,
    #[error("TEDSSensorDataError")]
    TEDSSensorDataError = -200826,
    #[error("DataSizeMoreThanSizeOfEEPROMOnTEDS")]
    DataSizeMoreThanSizeOfEEPROMOnTEDS = -200825,
    #[error("PROMOnTEDSContainsBasicTEDSData")]
    PROMOnTEDSContainsBasicTEDSData = -200824,
    #[error("PROMOnTEDSAlreadyWritten")]
    PROMOnTEDSAlreadyWritten = -200823,
    #[error("TEDSDoesNotContainPROM")]
    TEDSDoesNotContainPROM = -200822,
    #[error("HWTimedSinglePointNotSupportedAI")]
    HWTimedSinglePointNotSupportedAI = -200821,
    #[error("HWTimedSinglePointOddNumChansInAITask")]
    HWTimedSinglePointOddNumChansInAITask = -200820,
    #[error("CantUseOnlyOnBoardMemWithProgrammedIO")]
    CantUseOnlyOnBoardMemWithProgrammedIO = -200819,
    #[error("SwitchDevShutDownDueToHighTemp")]
    SwitchDevShutDownDueToHighTemp = -200818,
    #[error("ExcitationNotSupportedWhenTermCfgDiff")]
    ExcitationNotSupportedWhenTermCfgDiff = -200817,
    #[error("TEDSMinElecValGEMaxElecVal")]
    TEDSMinElecValGEMaxElecVal = -200816,
    #[error("TEDSMinPhysValGEMaxPhysVal")]
    TEDSMinPhysValGEMaxPhysVal = -200815,
    #[error("CIOnboardClockNotSupportedAsInputTerm")]
    CIOnboardClockNotSupportedAsInputTerm = -200814,
    #[error("InvalidSampModeForPositionMeas")]
    InvalidSampModeForPositionMeas = -200813,
    #[error("TrigWhenAOHWTimedSinglePtSampMode")]
    TrigWhenAOHWTimedSinglePtSampMode = -200812,
    #[error("DAQmxCantUseStringDueToUnknownChar")]
    DAQmxCantUseStringDueToUnknownChar = -200811,
    #[error("DAQmxCantRetrieveStringDueToUnknownChar")]
    DAQmxCantRetrieveStringDueToUnknownChar = -200810,
    #[error("ClearTEDSNotSupportedOnRT")]
    ClearTEDSNotSupportedOnRT = -200809,
    #[error("CfgTEDSNotSupportedOnRT")]
    CfgTEDSNotSupportedOnRT = -200808,
    #[error("ProgFilterClkCfgdToDifferentMinPulseWidthBySameTask1PerDev")]
    ProgFilterClkCfgdToDifferentMinPulseWidthBySameTask1PerDev = -200807,
    #[error("ProgFilterClkCfgdToDifferentMinPulseWidthByAnotherTask1PerDev")]
    ProgFilterClkCfgdToDifferentMinPulseWidthByAnotherTask1PerDev = -200806,
    #[error("NoLastExtCalDateTimeLastExtCalNotDAQmx")]
    NoLastExtCalDateTimeLastExtCalNotDAQmx = -200804,
    #[error("CannotWriteNotStartedAutoStartFalseNotOnDemandHWTimedSglPt")]
    CannotWriteNotStartedAutoStartFalseNotOnDemandHWTimedSglPt = -200803,
    #[error("CannotWriteNotStartedAutoStartFalseNotOnDemandBufSizeZero")]
    CannotWriteNotStartedAutoStartFalseNotOnDemandBufSizeZero = -200802,
    #[error("COInvalidTimingSrcDueToSignal")]
    COInvalidTimingSrcDueToSignal = -200801,
    #[error("CIInvalidTimingSrcForSampClkDueToSampTimingType")]
    CIInvalidTimingSrcForSampClkDueToSampTimingType = -200800,
    #[error("CIInvalidTimingSrcForEventCntDueToSampMode")]
    CIInvalidTimingSrcForEventCntDueToSampMode = -200799,
    #[error("NoChangeDetectOnNonInputDigLineForDev")]
    NoChangeDetectOnNonInputDigLineForDev = -200798,
    #[error("EmptyStringTermNameNotSupported")]
    EmptyStringTermNameNotSupported = -200797,
    #[error("MemMapEnabledForHWTimedNonBufferedAO")]
    MemMapEnabledForHWTimedNonBufferedAO = -200796,
    #[error("DevOnboardMemOverflowDuringHWTimedNonBufferedGen")]
    DevOnboardMemOverflowDuringHWTimedNonBufferedGen = -200795,
    #[error("CODAQmxWriteMultipleChans")]
    CODAQmxWriteMultipleChans = -200794,
    #[error("CantMaintainExistingValueAOSync")]
    CantMaintainExistingValueAOSync = -200793,
    #[error("MStudioMultiplePhysChansNotSupported")]
    MStudioMultiplePhysChansNotSupported = -200792,
    #[error("CantConfigureTEDSForChan")]
    CantConfigureTEDSForChan = -200791,
    #[error("WriteDataTypeTooSmall")]
    WriteDataTypeTooSmall = -200790,
    #[error("ReadDataTypeTooSmall")]
    ReadDataTypeTooSmall = -200789,
    #[error("MeasuredBridgeOffsetTooHigh")]
    MeasuredBridgeOffsetTooHigh = -200788,
    #[error("StartTrigConflictWithCOHWTimedSinglePt")]
    StartTrigConflictWithCOHWTimedSinglePt = -200787,
    #[error("SampClkRateExtSampClkTimebaseRateMismatch")]
    SampClkRateExtSampClkTimebaseRateMismatch = -200786,
    #[error("InvalidTimingSrcDueToSampTimingType")]
    InvalidTimingSrcDueToSampTimingType = -200785,
    #[error("VirtualTEDSFileNotFound")]
    VirtualTEDSFileNotFound = -200784,
    #[error("MStudioNoForwardPolyScaleCoeffs")]
    MStudioNoForwardPolyScaleCoeffs = -200783,
    #[error("MStudioNoReversePolyScaleCoeffs")]
    MStudioNoReversePolyScaleCoeffs = -200782,
    #[error("MStudioNoPolyScaleCoeffsUseCalc")]
    MStudioNoPolyScaleCoeffsUseCalc = -200781,
    #[error("MStudioNoForwardPolyScaleCoeffsUseCalc")]
    MStudioNoForwardPolyScaleCoeffsUseCalc = -200780,
    #[error("MStudioNoReversePolyScaleCoeffsUseCalc")]
    MStudioNoReversePolyScaleCoeffsUseCalc = -200779,
    #[error("COSampModeSampTimingTypeSampClkConflict")]
    COSampModeSampTimingTypeSampClkConflict = -200778,
    #[error("DevCannotProduceMinPulseWidth")]
    DevCannotProduceMinPulseWidth = -200777,
    #[error("CannotProduceMinPulseWidthGivenPropertyValues")]
    CannotProduceMinPulseWidthGivenPropertyValues = -200776,
    #[error("TermCfgdToDifferentMinPulseWidthByAnotherTask")]
    TermCfgdToDifferentMinPulseWidthByAnotherTask = -200775,
    #[error("TermCfgdToDifferentMinPulseWidthByAnotherProperty")]
    TermCfgdToDifferentMinPulseWidthByAnotherProperty = -200774,
    #[error("DigSyncNotAvailableOnTerm")]
    DigSyncNotAvailableOnTerm = -200773,
    #[error("DigFilterNotAvailableOnTerm")]
    DigFilterNotAvailableOnTerm = -200772,
    #[error("DigFilterEnabledMinPulseWidthNotCfg")]
    DigFilterEnabledMinPulseWidthNotCfg = -200771,
    #[error("DigFilterAndSyncBothEnabled")]
    DigFilterAndSyncBothEnabled = -200770,
    #[error("HWTimedSinglePointAOAndDataXferNotProgIO")]
    HWTimedSinglePointAOAndDataXferNotProgIO = -200769,
    #[error("NonBufferedAOAndDataXferNotProgIO")]
    NonBufferedAOAndDataXferNotProgIO = -200768,
    #[error("ProgIODataXferForBufferedAO")]
    ProgIODataXferForBufferedAO = -200767,
    #[error("TEDSLegacyTemplateIDInvalidOrUnsupported")]
    TEDSLegacyTemplateIDInvalidOrUnsupported = -200766,
    #[error("TEDSMappingMethodInvalidOrUnsupported")]
    TEDSMappingMethodInvalidOrUnsupported = -200765,
    #[error("TEDSLinearMappingSlopeZero")]
    TEDSLinearMappingSlopeZero = -200764,
    #[error("AIInputBufferSizeNotMultOfXferSize")]
    AIInputBufferSizeNotMultOfXferSize = -200763,
    #[error("NoSyncPulseExtSampClkTimebase")]
    NoSyncPulseExtSampClkTimebase = -200762,
    #[error("NoSyncPulseAnotherTaskRunning")]
    NoSyncPulseAnotherTaskRunning = -200761,
    #[error("AOMinMaxNotInGainRange")]
    AOMinMaxNotInGainRange = -200760,
    #[error("AOMinMaxNotInDACRange")]
    AOMinMaxNotInDACRange = -200759,
    #[error("DevOnlySupportsSampClkTimingAO")]
    DevOnlySupportsSampClkTimingAO = -200758,
    #[error("DevOnlySupportsSampClkTimingAI")]
    DevOnlySupportsSampClkTimingAI = -200757,
    #[error("TEDSIncompatibleSensorAndMeasType")]
    TEDSIncompatibleSensorAndMeasType = -200756,
    #[error("TEDSMultipleCalTemplatesNotSupported")]
    TEDSMultipleCalTemplatesNotSupported = -200755,
    #[error("TEDSTemplateParametersNotSupported")]
    TEDSTemplateParametersNotSupported = -200754,
    #[error("ParsingTEDSData")]
    ParsingTEDSData = -200753,
    #[error("MultipleActivePhysChansNotSupported")]
    MultipleActivePhysChansNotSupported = -200752,
    #[error("NoChansSpecdForChangeDetect")]
    NoChansSpecdForChangeDetect = -200751,
    #[error("InvalidCalVoltageForGivenGain")]
    InvalidCalVoltageForGivenGain = -200750,
    #[error("InvalidCalGain")]
    InvalidCalGain = -200749,
    #[error("MultipleWritesBetweenSampClks")]
    MultipleWritesBetweenSampClks = -200748,
    #[error("InvalidAcqTypeForFREQOUT")]
    InvalidAcqTypeForFREQOUT = -200747,
    #[error("SuitableTimebaseNotFoundTimeCombo2")]
    SuitableTimebaseNotFoundTimeCombo2 = -200746,
    #[error("SuitableTimebaseNotFoundFrequencyCombo2")]
    SuitableTimebaseNotFoundFrequencyCombo2 = -200745,
    #[error("RefClkRateRefClkSrcMismatch")]
    RefClkRateRefClkSrcMismatch = -200744,
    #[error("NoTEDSTerminalBlock")]
    NoTEDSTerminalBlock = -200743,
    #[error("CorruptedTEDSMemory")]
    CorruptedTEDSMemory = -200742,
    #[error("TEDSNotSupported")]
    TEDSNotSupported = -200741,
    #[error("TimingSrcTaskStartedBeforeTimedLoop")]
    TimingSrcTaskStartedBeforeTimedLoop = -200740,
    #[error("PropertyNotSupportedForTimingSrc")]
    PropertyNotSupportedForTimingSrc = -200739,
    #[error("TimingSrcDoesNotExist")]
    TimingSrcDoesNotExist = -200738,
    #[error("InputBufferSizeNotEqualSampsPerChanForFiniteSampMode")]
    InputBufferSizeNotEqualSampsPerChanForFiniteSampMode = -200737,
    #[error("FREQOUTCannotProduceDesiredFrequency2")]
    FREQOUTCannotProduceDesiredFrequency2 = -200736,
    #[error("ExtRefClkRateNotSpecified")]
    ExtRefClkRateNotSpecified = -200735,
    #[error("DeviceDoesNotSupportDMADataXferForNonBufferedAcq")]
    DeviceDoesNotSupportDMADataXferForNonBufferedAcq = -200734,
    #[error("DigFilterMinPulseWidthSetWhenTristateIsFalse")]
    DigFilterMinPulseWidthSetWhenTristateIsFalse = -200733,
    #[error("DigFilterEnableSetWhenTristateIsFalse")]
    DigFilterEnableSetWhenTristateIsFalse = -200732,
    #[error("NoHWTimingWithOnDemand")]
    NoHWTimingWithOnDemand = -200731,
    #[error("CannotDetectChangesWhenTristateIsFalse")]
    CannotDetectChangesWhenTristateIsFalse = -200730,
    #[error("CannotHandshakeWhenTristateIsFalse")]
    CannotHandshakeWhenTristateIsFalse = -200729,
    #[error("LinesUsedForStaticInputNotForHandshakingControl")]
    LinesUsedForStaticInputNotForHandshakingControl = -200728,
    #[error("LinesUsedForHandshakingControlNotForStaticInput")]
    LinesUsedForHandshakingControlNotForStaticInput = -200727,
    #[error("LinesUsedForStaticInputNotForHandshakingInput")]
    LinesUsedForStaticInputNotForHandshakingInput = -200726,
    #[error("LinesUsedForHandshakingInputNotForStaticInput")]
    LinesUsedForHandshakingInputNotForStaticInput = -200725,
    #[error("DifferentDITristateValsForChansInTask")]
    DifferentDITristateValsForChansInTask = -200724,
    #[error("TimebaseCalFreqVarianceTooLarge")]
    TimebaseCalFreqVarianceTooLarge = -200723,
    #[error("TimebaseCalFailedToConverge")]
    TimebaseCalFailedToConverge = -200722,
    #[error("InadequateResolutionForTimebaseCal")]
    InadequateResolutionForTimebaseCal = -200721,
    #[error("InvalidAOGainCalConst")]
    InvalidAOGainCalConst = -200720,
    #[error("InvalidAOOffsetCalConst")]
    InvalidAOOffsetCalConst = -200719,
    #[error("InvalidAIGainCalConst")]
    InvalidAIGainCalConst = -200718,
    #[error("InvalidAIOffsetCalConst")]
    InvalidAIOffsetCalConst = -200717,
    #[error("DigOutputOverrun")]
    DigOutputOverrun = -200716,
    #[error("DigInputOverrun")]
    DigInputOverrun = -200715,
    #[error("AcqStoppedDriverCantXferDataFastEnough")]
    AcqStoppedDriverCantXferDataFastEnough = -200714,
    #[error("ChansCantAppearInSameTask")]
    ChansCantAppearInSameTask = -200713,
    #[error("InputCfgFailedBecauseWatchdogExpired")]
    InputCfgFailedBecauseWatchdogExpired = -200712,
    #[error("AnalogTrigChanNotExternal")]
    AnalogTrigChanNotExternal = -200711,
    #[error("TooManyChansForInternalAIInputSrc")]
    TooManyChansForInternalAIInputSrc = -200710,
    #[error("TEDSSensorNotDetected")]
    TEDSSensorNotDetected = -200709,
    #[error("PrptyGetSpecdActiveItemFailedDueToDifftValues")]
    PrptyGetSpecdActiveItemFailedDueToDifftValues = -200708,
    #[error("RoutingDestTermPXIClk10InNotInSlot2")]
    RoutingDestTermPXIClk10InNotInSlot2 = -200706,
    #[error("RoutingDestTermPXIStarXNotInSlot2")]
    RoutingDestTermPXIStarXNotInSlot2 = -200705,
    #[error("RoutingSrcTermPXIStarXNotInSlot2")]
    RoutingSrcTermPXIStarXNotInSlot2 = -200704,
    #[error("RoutingSrcTermPXIStarInSlot16AndAbove")]
    RoutingSrcTermPXIStarInSlot16AndAbove = -200703,
    #[error("RoutingDestTermPXIStarInSlot16AndAbove")]
    RoutingDestTermPXIStarInSlot16AndAbove = -200702,
    #[error("RoutingDestTermPXIStarInSlot2")]
    RoutingDestTermPXIStarInSlot2 = -200701,
    #[error("RoutingSrcTermPXIStarInSlot2")]
    RoutingSrcTermPXIStarInSlot2 = -200700,
    #[error("RoutingDestTermPXIChassisNotIdentified")]
    RoutingDestTermPXIChassisNotIdentified = -200699,
    #[error("RoutingSrcTermPXIChassisNotIdentified")]
    RoutingSrcTermPXIChassisNotIdentified = -200698,
    #[error("FailedToAcquireCalData")]
    FailedToAcquireCalData = -200697,
    #[error("BridgeOffsetNullingCalNotSupported")]
    BridgeOffsetNullingCalNotSupported = -200696,
    #[error("AIMaxNotSpecified")]
    AIMaxNotSpecified = -200695,
    #[error("AIMinNotSpecified")]
    AIMinNotSpecified = -200694,
    #[error("OddTotalBufferSizeToWrite")]
    OddTotalBufferSizeToWrite = -200693,
    #[error("OddTotalNumSampsToWrite")]
    OddTotalNumSampsToWrite = -200692,
    #[error("BufferWithWaitMode")]
    BufferWithWaitMode = -200691,
    #[error("BufferWithHWTimedSinglePointSampMode")]
    BufferWithHWTimedSinglePointSampMode = -200690,
    #[error("COWritePulseLowTicksNotSupported")]
    COWritePulseLowTicksNotSupported = -200689,
    #[error("COWritePulseHighTicksNotSupported")]
    COWritePulseHighTicksNotSupported = -200688,
    #[error("COWritePulseLowTimeOutOfRange")]
    COWritePulseLowTimeOutOfRange = -200687,
    #[error("COWritePulseHighTimeOutOfRange")]
    COWritePulseHighTimeOutOfRange = -200686,
    #[error("COWriteFreqOutOfRange")]
    COWriteFreqOutOfRange = -200685,
    #[error("COWriteDutyCycleOutOfRange")]
    COWriteDutyCycleOutOfRange = -200684,
    #[error("InvalidInstallation")]
    InvalidInstallation = -200683,
    #[error("RefTrigMasterSessionUnavailable")]
    RefTrigMasterSessionUnavailable = -200682,
    #[error("RouteFailedBecauseWatchdogExpired")]
    RouteFailedBecauseWatchdogExpired = -200681,
    #[error("DeviceShutDownDueToHighTemp")]
    DeviceShutDownDueToHighTemp = -200680,
    #[error("NoMemMapWhenHWTimedSinglePoint")]
    NoMemMapWhenHWTimedSinglePoint = -200679,
    #[error("WriteFailedBecauseWatchdogExpired")]
    WriteFailedBecauseWatchdogExpired = -200678,
    #[error("DifftInternalAIInputSrcs")]
    DifftInternalAIInputSrcs = -200677,
    #[error("DifftAIInputSrcInOneChanGroup")]
    DifftAIInputSrcInOneChanGroup = -200676,
    #[error("InternalAIInputSrcInMultChanGroups")]
    InternalAIInputSrcInMultChanGroups = -200675,
    #[error("SwitchOpFailedDueToPrevError")]
    SwitchOpFailedDueToPrevError = -200674,
    #[error("WroteMultiSampsUsingSingleSampWrite")]
    WroteMultiSampsUsingSingleSampWrite = -200673,
    #[error("MismatchedInputArraySizes")]
    MismatchedInputArraySizes = -200672,
    #[error("CantExceedRelayDriveLimit")]
    CantExceedRelayDriveLimit = -200671,
    #[error("DACRngLowNotEqualToMinusRefVal")]
    DACRngLowNotEqualToMinusRefVal = -200670,
    #[error("CantAllowConnectDACToGnd")]
    CantAllowConnectDACToGnd = -200669,
    #[error("WatchdogTimeoutOutOfRangeAndNotSpecialVal")]
    WatchdogTimeoutOutOfRangeAndNotSpecialVal = -200668,
    #[error("NoWatchdogOutputOnPortReservedForInput")]
    NoWatchdogOutputOnPortReservedForInput = -200667,
    #[error("NoInputOnPortCfgdForWatchdogOutput")]
    NoInputOnPortCfgdForWatchdogOutput = -200666,
    #[error("WatchdogExpirationStateNotEqualForLinesInPort")]
    WatchdogExpirationStateNotEqualForLinesInPort = -200665,
    #[error("CannotPerformOpWhenTaskNotReserved")]
    CannotPerformOpWhenTaskNotReserved = -200664,
    #[error("PowerupStateNotSupported")]
    PowerupStateNotSupported = -200663,
    #[error("WatchdogTimerNotSupported")]
    WatchdogTimerNotSupported = -200662,
    #[error("OpNotSupportedWhenRefClkSrcNone")]
    OpNotSupportedWhenRefClkSrcNone = -200661,
    #[error("SampClkRateUnavailable")]
    SampClkRateUnavailable = -200660,
    #[error("PrptyGetSpecdSingleActiveChanFailedDueToDifftVals")]
    PrptyGetSpecdSingleActiveChanFailedDueToDifftVals = -200659,
    #[error("PrptyGetImpliedActiveChanFailedDueToDifftVals")]
    PrptyGetImpliedActiveChanFailedDueToDifftVals = -200658,
    #[error("PrptyGetSpecdActiveChanFailedDueToDifftVals")]
    PrptyGetSpecdActiveChanFailedDueToDifftVals = -200657,
    #[error("NoRegenWhenUsingBrdMem")]
    NoRegenWhenUsingBrdMem = -200656,
    #[error("NonbufferedReadMoreThanSampsPerChan")]
    NonbufferedReadMoreThanSampsPerChan = -200655,
    #[error("WatchdogExpirationTristateNotSpecdForEntirePort")]
    WatchdogExpirationTristateNotSpecdForEntirePort = -200654,
    #[error("PowerupTristateNotSpecdForEntirePort")]
    PowerupTristateNotSpecdForEntirePort = -200653,
    #[error("PowerupStateNotSpecdForEntirePort")]
    PowerupStateNotSpecdForEntirePort = -200652,
    #[error("CantSetWatchdogExpirationOnDigInChan")]
    CantSetWatchdogExpirationOnDigInChan = -200651,
    #[error("CantSetPowerupStateOnDigInChan")]
    CantSetPowerupStateOnDigInChan = -200650,
    #[error("PhysChanNotInTask")]
    PhysChanNotInTask = -200649,
    #[error("PhysChanDevNotInTask")]
    PhysChanDevNotInTask = -200648,
    #[error("DigInputNotSupported")]
    DigInputNotSupported = -200647,
    #[error("DigFilterIntervalNotEqualForLines")]
    DigFilterIntervalNotEqualForLines = -200646,
    #[error("DigFilterIntervalAlreadyCfgd")]
    DigFilterIntervalAlreadyCfgd = -200645,
    #[error("CantResetExpiredWatchdog")]
    CantResetExpiredWatchdog = -200644,
    #[error("ActiveChanTooManyLinesSpecdWhenGettingPrpty")]
    ActiveChanTooManyLinesSpecdWhenGettingPrpty = -200643,
    #[error("ActiveChanNotSpecdWhenGetting1LinePrpty")]
    ActiveChanNotSpecdWhenGetting1LinePrpty = -200642,
    #[error("DigPrptyCannotBeSetPerLine")]
    DigPrptyCannotBeSetPerLine = -200641,
    #[error("SendAdvCmpltAfterWaitForTrigInScanlist")]
    SendAdvCmpltAfterWaitForTrigInScanlist = -200640,
    #[error("DisconnectionRequiredInScanlist")]
    DisconnectionRequiredInScanlist = -200639,
    #[error("TwoWaitForTrigsAfterConnectionInScanlist")]
    TwoWaitForTrigsAfterConnectionInScanlist = -200638,
    #[error("ActionSeparatorRequiredAfterBreakingConnectionInScanlist")]
    ActionSeparatorRequiredAfterBreakingConnectionInScanlist = -200637,
    #[error("ConnectionInScanlistMustWaitForTrig")]
    ConnectionInScanlistMustWaitForTrig = -200636,
    #[error("ActionNotSupportedTaskNotWatchdog")]
    ActionNotSupportedTaskNotWatchdog = -200635,
    #[error("WfmNameSameAsScriptName")]
    WfmNameSameAsScriptName = -200634,
    #[error("ScriptNameSameAsWfmName")]
    ScriptNameSameAsWfmName = -200633,
    #[error("DSFStopClock")]
    DSFStopClock = -200632,
    #[error("DSFReadyForStartClock")]
    DSFReadyForStartClock = -200631,
    #[error("WriteOffsetNotMultOfIncr")]
    WriteOffsetNotMultOfIncr = -200630,
    #[error("DifferentPrptyValsNotSupportedOnDev")]
    DifferentPrptyValsNotSupportedOnDev = -200629,
    #[error("RefAndPauseTrigConfigured")]
    RefAndPauseTrigConfigured = -200628,
    #[error("FailedToEnableHighSpeedInputClock")]
    FailedToEnableHighSpeedInputClock = -200627,
    #[error("EmptyPhysChanInPowerUpStatesArray")]
    EmptyPhysChanInPowerUpStatesArray = -200626,
    #[error("ActivePhysChanTooManyLinesSpecdWhenGettingPrpty")]
    ActivePhysChanTooManyLinesSpecdWhenGettingPrpty = -200625,
    #[error("ActivePhysChanNotSpecdWhenGetting1LinePrpty")]
    ActivePhysChanNotSpecdWhenGetting1LinePrpty = -200624,
    #[error("PXIDevTempCausedShutDown")]
    PXIDevTempCausedShutDown = -200623,
    #[error("InvalidNumSampsToWrite")]
    InvalidNumSampsToWrite = -200622,
    #[error("OutputFIFOUnderflow2")]
    OutputFIFOUnderflow2 = -200621,
    #[error("RepeatedAIPhysicalChan")]
    RepeatedAIPhysicalChan = -200620,
    #[error("MultScanOpsInOneChassis")]
    MultScanOpsInOneChassis = -200619,
    #[error("InvalidAIChanOrder")]
    InvalidAIChanOrder = -200618,
    #[error("ReversePowerProtectionActivated")]
    ReversePowerProtectionActivated = -200617,
    #[error("InvalidAsynOpHandle")]
    InvalidAsynOpHandle = -200616,
    #[error("FailedToEnableHighSpeedOutput")]
    FailedToEnableHighSpeedOutput = -200615,
    #[error("CannotReadPastEndOfRecord")]
    CannotReadPastEndOfRecord = -200614,
    #[error("AcqStoppedToPreventInputBufferOverwriteOneDataXferMech")]
    AcqStoppedToPreventInputBufferOverwriteOneDataXferMech = -200613,
    #[error("ZeroBasedChanIndexInvalid")]
    ZeroBasedChanIndexInvalid = -200612,
    #[error("NoChansOfGivenTypeInTask")]
    NoChansOfGivenTypeInTask = -200611,
    #[error("SampClkSrcInvalidForOutputValidForInput")]
    SampClkSrcInvalidForOutputValidForInput = -200610,
    #[error("OutputBufSizeTooSmallToStartGen")]
    OutputBufSizeTooSmallToStartGen = -200609,
    #[error("InputBufSizeTooSmallToStartAcq")]
    InputBufSizeTooSmallToStartAcq = -200608,
    #[error("ExportTwoSignalsOnSameTerminal")]
    ExportTwoSignalsOnSameTerminal = -200607,
    #[error("ChanIndexInvalid")]
    ChanIndexInvalid = -200606,
    #[error("RangeSyntaxNumberTooBig")]
    RangeSyntaxNumberTooBig = -200605,
    #[error("NULLPtr")]
    NULLPtr = -200604,
    #[error("ScaledMinEqualMax")]
    ScaledMinEqualMax = -200603,
    #[error("PreScaledMinEqualMax")]
    PreScaledMinEqualMax = -200602,
    #[error("PropertyNotSupportedForScaleType")]
    PropertyNotSupportedForScaleType = -200601,
    #[error("ChannelNameGenerationNumberTooBig")]
    ChannelNameGenerationNumberTooBig = -200600,
    #[error("RepeatedNumberInScaledValues")]
    RepeatedNumberInScaledValues = -200599,
    #[error("RepeatedNumberInPreScaledValues")]
    RepeatedNumberInPreScaledValues = -200598,
    #[error("LinesAlreadyReservedForOutput")]
    LinesAlreadyReservedForOutput = -200597,
    #[error("SwitchOperationChansSpanMultipleDevsInList")]
    SwitchOperationChansSpanMultipleDevsInList = -200596,
    #[error("InvalidIDInListAtBeginningOfSwitchOperation")]
    InvalidIDInListAtBeginningOfSwitchOperation = -200595,
    #[error("MStudioInvalidPolyDirection")]
    MStudioInvalidPolyDirection = -200594,
    #[error("MStudioPropertyGetWhileTaskNotVerified")]
    MStudioPropertyGetWhileTaskNotVerified = -200593,
    #[error("RangeWithTooManyObjects")]
    RangeWithTooManyObjects = -200592,
    #[error("CppDotNetAPINegativeBufferSize")]
    CppDotNetAPINegativeBufferSize = -200591,
    #[error("CppCantRemoveInvalidEventHandler")]
    CppCantRemoveInvalidEventHandler = -200590,
    #[error("CppCantRemoveEventHandlerTwice")]
    CppCantRemoveEventHandlerTwice = -200589,
    #[error("CppCantRemoveOtherObjectsEventHandler")]
    CppCantRemoveOtherObjectsEventHandler = -200588,
    #[error("DigLinesReservedOrUnavailable")]
    DigLinesReservedOrUnavailable = -200587,
    #[error("DSFFailedToResetStream")]
    DSFFailedToResetStream = -200586,
    #[error("DSFReadyForOutputNotAsserted")]
    DSFReadyForOutputNotAsserted = -200585,
    #[error("SampToWritePerChanNotMultipleOfIncr")]
    SampToWritePerChanNotMultipleOfIncr = -200584,
    #[error("AOPropertiesCauseVoltageBelowMin")]
    AOPropertiesCauseVoltageBelowMin = -200583,
    #[error("AOPropertiesCauseVoltageOverMax")]
    AOPropertiesCauseVoltageOverMax = -200582,
    #[error("PropertyNotSupportedWhenRefClkSrcNone")]
    PropertyNotSupportedWhenRefClkSrcNone = -200581,
    #[error("AIMaxTooSmall")]
    AIMaxTooSmall = -200580,
    #[error("AIMaxTooLarge")]
    AIMaxTooLarge = -200579,
    #[error("AIMinTooSmall")]
    AIMinTooSmall = -200578,
    #[error("AIMinTooLarge")]
    AIMinTooLarge = -200577,
    #[error("BuiltInCJCSrcNotSupported")]
    BuiltInCJCSrcNotSupported = -200576,
    #[error("TooManyPostTrigSampsPerChan")]
    TooManyPostTrigSampsPerChan = -200575,
    #[error("TrigLineNotFoundSingleDevRoute")]
    TrigLineNotFoundSingleDevRoute = -200574,
    #[error("DifferentInternalAIInputSources")]
    DifferentInternalAIInputSources = -200573,
    #[error("DifferentAIInputSrcInOneChanGroup")]
    DifferentAIInputSrcInOneChanGroup = -200572,
    #[error("InternalAIInputSrcInMultipleChanGroups")]
    InternalAIInputSrcInMultipleChanGroups = -200571,
    #[error("CAPIChanIndexInvalid")]
    CAPIChanIndexInvalid = -200570,
    #[error("CollectionDoesNotMatchChanType")]
    CollectionDoesNotMatchChanType = -200569,
    #[error("OutputCantStartChangedRegenerationMode")]
    OutputCantStartChangedRegenerationMode = -200568,
    #[error("OutputCantStartChangedBufferSize")]
    OutputCantStartChangedBufferSize = -200567,
    #[error("ChanSizeTooBigForU32PortWrite")]
    ChanSizeTooBigForU32PortWrite = -200566,
    #[error("ChanSizeTooBigForU8PortWrite")]
    ChanSizeTooBigForU8PortWrite = -200565,
    #[error("ChanSizeTooBigForU32PortRead")]
    ChanSizeTooBigForU32PortRead = -200564,
    #[error("ChanSizeTooBigForU8PortRead")]
    ChanSizeTooBigForU8PortRead = -200563,
    #[error("InvalidDigDataWrite")]
    InvalidDigDataWrite = -200562,
    #[error("InvalidAODataWrite")]
    InvalidAODataWrite = -200561,
    #[error("WaitUntilDoneDoesNotIndicateDone")]
    WaitUntilDoneDoesNotIndicateDone = -200560,
    #[error("MultiChanTypesInTask")]
    MultiChanTypesInTask = -200559,
    #[error("MultiDevsInTask")]
    MultiDevsInTask = -200558,
    #[error("CannotSetPropertyWhenTaskRunning")]
    CannotSetPropertyWhenTaskRunning = -200557,
    #[error("CannotGetPropertyWhenTaskNotCommittedOrRunning")]
    CannotGetPropertyWhenTaskNotCommittedOrRunning = -200556,
    #[error("LeadingUnderscoreInString")]
    LeadingUnderscoreInString = -200555,
    #[error("TrailingSpaceInString")]
    TrailingSpaceInString = -200554,
    #[error("LeadingSpaceInString")]
    LeadingSpaceInString = -200553,
    #[error("InvalidCharInString")]
    InvalidCharInString = -200552,
    #[error("DLLBecameUnlocked")]
    DLLBecameUnlocked = -200551,
    #[error("DLLLock")]
    DLLLock = -200550,
    #[error("SelfCalConstsInvalid")]
    SelfCalConstsInvalid = -200549,
    #[error("InvalidTrigCouplingExceptForExtTrigChan")]
    InvalidTrigCouplingExceptForExtTrigChan = -200548,
    #[error("WriteFailsBufferSizeAutoConfigured")]
    WriteFailsBufferSizeAutoConfigured = -200547,
    #[error("ExtCalAdjustExtRefVoltageFailed")]
    ExtCalAdjustExtRefVoltageFailed = -200546,
    #[error("SelfCalFailedExtNoiseOrRefVoltageOutOfCal")]
    SelfCalFailedExtNoiseOrRefVoltageOutOfCal = -200545,
    #[error("ExtCalTemperatureNotDAQmx")]
    ExtCalTemperatureNotDAQmx = -200544,
    #[error("ExtCalDateTimeNotDAQmx")]
    ExtCalDateTimeNotDAQmx = -200543,
    #[error("SelfCalTemperatureNotDAQmx")]
    SelfCalTemperatureNotDAQmx = -200542,
    #[error("SelfCalDateTimeNotDAQmx")]
    SelfCalDateTimeNotDAQmx = -200541,
    #[error("DACRefValNotSet")]
    DACRefValNotSet = -200540,
    #[error("AnalogMultiSampWriteNotSupported")]
    AnalogMultiSampWriteNotSupported = -200539,
    #[error("InvalidActionInControlTask")]
    InvalidActionInControlTask = -200538,
    #[error("PolyCoeffsInconsistent")]
    PolyCoeffsInconsistent = -200537,
    #[error("SensorValTooLow")]
    SensorValTooLow = -200536,
    #[error("SensorValTooHigh")]
    SensorValTooHigh = -200535,
    #[error("WaveformNameTooLong")]
    WaveformNameTooLong = -200534,
    #[error("IdentifierTooLongInScript")]
    IdentifierTooLongInScript = -200533,
    #[error("UnexpectedIDFollowingSwitchChanName")]
    UnexpectedIDFollowingSwitchChanName = -200532,
    #[error("RelayNameNotSpecifiedInList")]
    RelayNameNotSpecifiedInList = -200531,
    #[error("UnexpectedIDFollowingRelayNameInList")]
    UnexpectedIDFollowingRelayNameInList = -200530,
    #[error("UnexpectedIDFollowingSwitchOpInList")]
    UnexpectedIDFollowingSwitchOpInList = -200529,
    #[error("InvalidLineGrouping")]
    InvalidLineGrouping = -200528,
    #[error("CtrMinMax")]
    CtrMinMax = -200527,
    #[error("WriteChanTypeMismatch")]
    WriteChanTypeMismatch = -200526,
    #[error("ReadChanTypeMismatch")]
    ReadChanTypeMismatch = -200525,
    #[error("WriteNumChansMismatch")]
    WriteNumChansMismatch = -200524,
    #[error("OneChanReadForMultiChanTask")]
    OneChanReadForMultiChanTask = -200523,
    #[error("CannotSelfCalDuringExtCal")]
    CannotSelfCalDuringExtCal = -200522,
    #[error("MeasCalAdjustOscillatorPhaseDAC")]
    MeasCalAdjustOscillatorPhaseDAC = -200521,
    #[error("InvalidCalConstCalADCAdjustment")]
    InvalidCalConstCalADCAdjustment = -200520,
    #[error("InvalidCalConstOscillatorFreqDACValue")]
    InvalidCalConstOscillatorFreqDACValue = -200519,
    #[error("InvalidCalConstOscillatorPhaseDACValue")]
    InvalidCalConstOscillatorPhaseDACValue = -200518,
    #[error("InvalidCalConstOffsetDACValue")]
    InvalidCalConstOffsetDACValue = -200517,
    #[error("InvalidCalConstGainDACValue")]
    InvalidCalConstGainDACValue = -200516,
    #[error("InvalidNumCalADCReadsToAverage")]
    InvalidNumCalADCReadsToAverage = -200515,
    #[error("InvalidCfgCalAdjustDirectPathOutputImpedance")]
    InvalidCfgCalAdjustDirectPathOutputImpedance = -200514,
    #[error("InvalidCfgCalAdjustMainPathOutputImpedance")]
    InvalidCfgCalAdjustMainPathOutputImpedance = -200513,
    #[error("InvalidCfgCalAdjustMainPathPostAmpGainAndOffset")]
    InvalidCfgCalAdjustMainPathPostAmpGainAndOffset = -200512,
    #[error("InvalidCfgCalAdjustMainPathPreAmpGain")]
    InvalidCfgCalAdjustMainPathPreAmpGain = -200511,
    #[error("InvalidCfgCalAdjustMainPreAmpOffset")]
    InvalidCfgCalAdjustMainPreAmpOffset = -200510,
    #[error("MeasCalAdjustCalADC")]
    MeasCalAdjustCalADC = -200509,
    #[error("MeasCalAdjustOscillatorFrequency")]
    MeasCalAdjustOscillatorFrequency = -200508,
    #[error("MeasCalAdjustDirectPathOutputImpedance")]
    MeasCalAdjustDirectPathOutputImpedance = -200507,
    #[error("MeasCalAdjustMainPathOutputImpedance")]
    MeasCalAdjustMainPathOutputImpedance = -200506,
    #[error("MeasCalAdjustDirectPathGain")]
    MeasCalAdjustDirectPathGain = -200505,
    #[error("MeasCalAdjustMainPathPostAmpGainAndOffset")]
    MeasCalAdjustMainPathPostAmpGainAndOffset = -200504,
    #[error("MeasCalAdjustMainPathPreAmpGain")]
    MeasCalAdjustMainPathPreAmpGain = -200503,
    #[error("MeasCalAdjustMainPathPreAmpOffset")]
    MeasCalAdjustMainPathPreAmpOffset = -200502,
    #[error("InvalidDateTimeInEEPROM")]
    InvalidDateTimeInEEPROM = -200501,
    #[error("UnableToLocateErrorResources")]
    UnableToLocateErrorResources = -200500,
    #[error("DotNetAPINotUnsigned32BitNumber")]
    DotNetAPINotUnsigned32BitNumber = -200499,
    #[error("InvalidRangeOfObjectsSyntaxInString")]
    InvalidRangeOfObjectsSyntaxInString = -200498,
    #[error("AttemptToEnableLineNotPreviouslyDisabled")]
    AttemptToEnableLineNotPreviouslyDisabled = -200497,
    #[error("InvalidCharInPattern")]
    InvalidCharInPattern = -200496,
    #[error("IntermediateBufferFull")]
    IntermediateBufferFull = -200495,
    #[error("LoadTaskFailsBecauseNoTimingOnDev")]
    LoadTaskFailsBecauseNoTimingOnDev = -200494,
    #[error("CAPIReservedParamNotNULLNorEmpty")]
    CAPIReservedParamNotNULLNorEmpty = -200493,
    #[error("CAPIReservedParamNotNULL")]
    CAPIReservedParamNotNULL = -200492,
    #[error("CAPIReservedParamNotZero")]
    CAPIReservedParamNotZero = -200491,
    #[error("SampleValueOutOfRange")]
    SampleValueOutOfRange = -200490,
    #[error("ChanAlreadyInTask")]
    ChanAlreadyInTask = -200489,
    #[error("VirtualChanDoesNotExist")]
    VirtualChanDoesNotExist = -200488,
    #[error("ChanNotInTask")]
    ChanNotInTask = -200486,
    #[error("TaskNotInDataNeighborhood")]
    TaskNotInDataNeighborhood = -200485,
    #[error("CantSaveTaskWithoutReplace")]
    CantSaveTaskWithoutReplace = -200484,
    #[error("CantSaveChanWithoutReplace")]
    CantSaveChanWithoutReplace = -200483,
    #[error("DevNotInTask")]
    DevNotInTask = -200482,
    #[error("DevAlreadyInTask")]
    DevAlreadyInTask = -200481,
    #[error("CanNotPerformOpWhileTaskRunning")]
    CanNotPerformOpWhileTaskRunning = -200479,
    #[error("CanNotPerformOpWhenNoChansInTask")]
    CanNotPerformOpWhenNoChansInTask = -200478,
    #[error("CanNotPerformOpWhenNoDevInTask")]
    CanNotPerformOpWhenNoDevInTask = -200477,
    #[error("CannotPerformOpWhenTaskNotRunning")]
    CannotPerformOpWhenTaskNotRunning = -200475,
    #[error("OperationTimedOut")]
    OperationTimedOut = -200474,
    #[error("CannotReadWhenAutoStartFalseAndTaskNotRunningOrCommitted")]
    CannotReadWhenAutoStartFalseAndTaskNotRunningOrCommitted = -200473,
    #[error("CannotWriteWhenAutoStartFalseAndTaskNotRunningOrCommitted")]
    CannotWriteWhenAutoStartFalseAndTaskNotRunningOrCommitted = -200472,
    #[error("TaskVersionNew")]
    TaskVersionNew = -200470,
    #[error("ChanVersionNew")]
    ChanVersionNew = -200469,
    #[error("EmptyString")]
    EmptyString = -200467,
    #[error("ChannelSizeTooBigForPortReadType")]
    ChannelSizeTooBigForPortReadType = -200466,
    #[error("ChannelSizeTooBigForPortWriteType")]
    ChannelSizeTooBigForPortWriteType = -200465,
    #[error("ExpectedNumberOfChannelsVerificationFailed")]
    ExpectedNumberOfChannelsVerificationFailed = -200464,
    #[error("NumLinesMismatchInReadOrWrite")]
    NumLinesMismatchInReadOrWrite = -200463,
    #[error("OutputBufferEmpty")]
    OutputBufferEmpty = -200462,
    #[error("InvalidChanName")]
    InvalidChanName = -200461,
    #[error("ReadNoInputChansInTask")]
    ReadNoInputChansInTask = -200460,
    #[error("WriteNoOutputChansInTask")]
    WriteNoOutputChansInTask = -200459,
    #[error("PropertyNotSupportedNotInputTask")]
    PropertyNotSupportedNotInputTask = -200457,
    #[error("PropertyNotSupportedNotOutputTask")]
    PropertyNotSupportedNotOutputTask = -200456,
    #[error("GetPropertyNotInputBufferedTask")]
    GetPropertyNotInputBufferedTask = -200455,
    #[error("GetPropertyNotOutputBufferedTask")]
    GetPropertyNotOutputBufferedTask = -200454,
    #[error("InvalidTimeoutVal")]
    InvalidTimeoutVal = -200453,
    #[error("AttributeNotSupportedInTaskContext")]
    AttributeNotSupportedInTaskContext = -200452,
    #[error("AttributeNotQueryableUnlessTaskIsCommitted")]
    AttributeNotQueryableUnlessTaskIsCommitted = -200451,
    #[error("AttributeNotSettableWhenTaskIsRunning")]
    AttributeNotSettableWhenTaskIsRunning = -200450,
    #[error("DACRngLowNotMinusRefValNorZero")]
    DACRngLowNotMinusRefValNorZero = -200449,
    #[error("DACRngHighNotEqualRefVal")]
    DACRngHighNotEqualRefVal = -200448,
    #[error("UnitsNotFromCustomScale")]
    UnitsNotFromCustomScale = -200447,
    #[error("InvalidVoltageReadingDuringExtCal")]
    InvalidVoltageReadingDuringExtCal = -200446,
    #[error("CalFunctionNotSupported")]
    CalFunctionNotSupported = -200445,
    #[error("InvalidPhysicalChanForCal")]
    InvalidPhysicalChanForCal = -200444,
    #[error("ExtCalNotComplete")]
    ExtCalNotComplete = -200443,
    #[error("CantSyncToExtStimulusFreqDuringCal")]
    CantSyncToExtStimulusFreqDuringCal = -200442,
    #[error("UnableToDetectExtStimulusFreqDuringCal")]
    UnableToDetectExtStimulusFreqDuringCal = -200441,
    #[error("InvalidCloseAction")]
    InvalidCloseAction = -200440,
    #[error("ExtCalFunctionOutsideExtCalSession")]
    ExtCalFunctionOutsideExtCalSession = -200439,
    #[error("InvalidCalArea")]
    InvalidCalArea = -200438,
    #[error("ExtCalConstsInvalid")]
    ExtCalConstsInvalid = -200437,
    #[error("StartTrigDelayWithExtSampClk")]
    StartTrigDelayWithExtSampClk = -200436,
    #[error("DelayFromSampClkWithExtConv")]
    DelayFromSampClkWithExtConv = -200435,
    #[error("FewerThan2PreScaledVals")]
    FewerThan2PreScaledVals = -200434,
    #[error("FewerThan2ScaledValues")]
    FewerThan2ScaledValues = -200433,
    #[error("PhysChanOutputType")]
    PhysChanOutputType = -200432,
    #[error("PhysChanMeasType")]
    PhysChanMeasType = -200431,
    #[error("InvalidPhysChanType")]
    InvalidPhysChanType = -200430,
    #[error("LabVIEWEmptyTaskOrChans")]
    LabVIEWEmptyTaskOrChans = -200429,
    #[error("LabVIEWInvalidTaskOrChans")]
    LabVIEWInvalidTaskOrChans = -200428,
    #[error("InvalidRefClkRate")]
    InvalidRefClkRate = -200427,
    #[error("InvalidExtTrigImpedance")]
    InvalidExtTrigImpedance = -200426,
    #[error("HystTrigLevelAIMax")]
    HystTrigLevelAIMax = -200425,
    #[error("LineNumIncompatibleWithVideoSignalFormat")]
    LineNumIncompatibleWithVideoSignalFormat = -200424,
    #[error("TrigWindowAIMinAIMaxCombo")]
    TrigWindowAIMinAIMaxCombo = -200423,
    #[error("TrigAIMinAIMax")]
    TrigAIMinAIMax = -200422,
    #[error("HystTrigLevelAIMin")]
    HystTrigLevelAIMin = -200421,
    #[error("InvalidSampRateConsiderRIS")]
    InvalidSampRateConsiderRIS = -200420,
    #[error("InvalidReadPosDuringRIS")]
    InvalidReadPosDuringRIS = -200419,
    #[error("ImmedTrigDuringRISMode")]
    ImmedTrigDuringRISMode = -200418,
    #[error("TDCNotEnabledDuringRISMode")]
    TDCNotEnabledDuringRISMode = -200417,
    #[error("MultiRecWithRIS")]
    MultiRecWithRIS = -200416,
    #[error("InvalidRefClkSrc")]
    InvalidRefClkSrc = -200415,
    #[error("InvalidSampClkSrc")]
    InvalidSampClkSrc = -200414,
    #[error("InsufficientOnBoardMemForNumRecsAndSamps")]
    InsufficientOnBoardMemForNumRecsAndSamps = -200413,
    #[error("InvalidAIAttenuation")]
    InvalidAIAttenuation = -200412,
    #[error("ACCouplingNotAllowedWith50OhmImpedance")]
    ACCouplingNotAllowedWith50OhmImpedance = -200411,
    #[error("InvalidRecordNum")]
    InvalidRecordNum = -200410,
    #[error("ZeroSlopeLinearScale")]
    ZeroSlopeLinearScale = -200409,
    #[error("ZeroReversePolyScaleCoeffs")]
    ZeroReversePolyScaleCoeffs = -200408,
    #[error("ZeroForwardPolyScaleCoeffs")]
    ZeroForwardPolyScaleCoeffs = -200407,
    #[error("NoReversePolyScaleCoeffs")]
    NoReversePolyScaleCoeffs = -200406,
    #[error("NoForwardPolyScaleCoeffs")]
    NoForwardPolyScaleCoeffs = -200405,
    #[error("NoPolyScaleCoeffs")]
    NoPolyScaleCoeffs = -200404,
    #[error("ReversePolyOrderLessThanNumPtsToCompute")]
    ReversePolyOrderLessThanNumPtsToCompute = -200403,
    #[error("ReversePolyOrderNotPositive")]
    ReversePolyOrderNotPositive = -200402,
    #[error("NumPtsToComputeNotPositive")]
    NumPtsToComputeNotPositive = -200401,
    #[error("WaveformLengthNotMultipleOfIncr")]
    WaveformLengthNotMultipleOfIncr = -200400,
    #[error("CAPINoExtendedErrorInfoAvailable")]
    CAPINoExtendedErrorInfoAvailable = -200399,
    #[error("CVIFunctionNotFoundInDAQmxDLL")]
    CVIFunctionNotFoundInDAQmxDLL = -200398,
    #[error("CVIFailedToLoadDAQmxDLL")]
    CVIFailedToLoadDAQmxDLL = -200397,
    #[error("NoCommonTrigLineForImmedRoute")]
    NoCommonTrigLineForImmedRoute = -200396,
    #[error("NoCommonTrigLineForTaskRoute")]
    NoCommonTrigLineForTaskRoute = -200395,
    #[error("F64PrptyValNotUnsignedInt")]
    F64PrptyValNotUnsignedInt = -200394,
    #[error("RegisterNotWritable")]
    RegisterNotWritable = -200393,
    #[error("InvalidOutputVoltageAtSampClkRate")]
    InvalidOutputVoltageAtSampClkRate = -200392,
    #[error("StrobePhaseShiftDCMBecameUnlocked")]
    StrobePhaseShiftDCMBecameUnlocked = -200391,
    #[error("DrivePhaseShiftDCMBecameUnlocked")]
    DrivePhaseShiftDCMBecameUnlocked = -200390,
    #[error("ClkOutPhaseShiftDCMBecameUnlocked")]
    ClkOutPhaseShiftDCMBecameUnlocked = -200389,
    #[error("OutputBoardClkDCMBecameUnlocked")]
    OutputBoardClkDCMBecameUnlocked = -200388,
    #[error("InputBoardClkDCMBecameUnlocked")]
    InputBoardClkDCMBecameUnlocked = -200387,
    #[error("InternalClkDCMBecameUnlocked")]
    InternalClkDCMBecameUnlocked = -200386,
    #[error("DCMLock")]
    DCMLock = -200385,
    #[error("DataLineReservedForDynamicOutput")]
    DataLineReservedForDynamicOutput = -200384,
    #[error("InvalidRefClkSrcGivenSampClkSrc")]
    InvalidRefClkSrcGivenSampClkSrc = -200383,
    #[error("NoPatternMatcherAvailable")]
    NoPatternMatcherAvailable = -200382,
    #[error("InvalidDelaySampRateBelowPhaseShiftDCMThresh")]
    InvalidDelaySampRateBelowPhaseShiftDCMThresh = -200381,
    #[error("StrainGageCalibration")]
    StrainGageCalibration = -200380,
    #[error("InvalidExtClockFreqAndDivCombo")]
    InvalidExtClockFreqAndDivCombo = -200379,
    #[error("CustomScaleDoesNotExist")]
    CustomScaleDoesNotExist = -200378,
    #[error("OnlyFrontEndChanOpsDuringScan")]
    OnlyFrontEndChanOpsDuringScan = -200377,
    #[error("InvalidOptionForDigitalPortChannel")]
    InvalidOptionForDigitalPortChannel = -200376,
    #[error("UnsupportedSignalTypeExportSignal")]
    UnsupportedSignalTypeExportSignal = -200375,
    #[error("InvalidSignalTypeExportSignal")]
    InvalidSignalTypeExportSignal = -200374,
    #[error("UnsupportedTrigTypeSendsSWTrig")]
    UnsupportedTrigTypeSendsSWTrig = -200373,
    #[error("InvalidTrigTypeSendsSWTrig")]
    InvalidTrigTypeSendsSWTrig = -200372,
    #[error("RepeatedPhysicalChan")]
    RepeatedPhysicalChan = -200371,
    #[error("ResourcesInUseForRouteInTask")]
    ResourcesInUseForRouteInTask = -200370,
    #[error("ResourcesInUseForRoute")]
    ResourcesInUseForRoute = -200369,
    #[error("RouteNotSupportedByHW")]
    RouteNotSupportedByHW = -200368,
    #[error("ResourcesInUseForExportSignalPolarity")]
    ResourcesInUseForExportSignalPolarity = -200367,
    #[error("ResourcesInUseForInversionInTask")]
    ResourcesInUseForInversionInTask = -200366,
    #[error("ResourcesInUseForInversion")]
    ResourcesInUseForInversion = -200365,
    #[error("ExportSignalPolarityNotSupportedByHW")]
    ExportSignalPolarityNotSupportedByHW = -200364,
    #[error("InversionNotSupportedByHW")]
    InversionNotSupportedByHW = -200363,
    #[error("OverloadedChansExistNotRead")]
    OverloadedChansExistNotRead = -200362,
    #[error("InputFIFOOverflow2")]
    InputFIFOOverflow2 = -200361,
    #[error("CJCChanNotSpecd")]
    CJCChanNotSpecd = -200360,
    #[error("CtrExportSignalNotPossible")]
    CtrExportSignalNotPossible = -200359,
    #[error("RefTrigWhenContinuous")]
    RefTrigWhenContinuous = -200358,
    #[error("IncompatibleSensorOutputAndDeviceInputRanges")]
    IncompatibleSensorOutputAndDeviceInputRanges = -200357,
    #[error("CustomScaleNameUsed")]
    CustomScaleNameUsed = -200356,
    #[error("PropertyValNotSupportedByHW")]
    PropertyValNotSupportedByHW = -200355,
    #[error("PropertyValNotValidTermName")]
    PropertyValNotValidTermName = -200354,
    #[error("ResourcesInUseForProperty")]
    ResourcesInUseForProperty = -200353,
    #[error("CJCChanAlreadyUsed")]
    CJCChanAlreadyUsed = -200352,
    #[error("ForwardPolynomialCoefNotSpecd")]
    ForwardPolynomialCoefNotSpecd = -200351,
    #[error("TableScaleNumPreScaledAndScaledValsNotEqual")]
    TableScaleNumPreScaledAndScaledValsNotEqual = -200350,
    #[error("TableScalePreScaledValsNotSpecd")]
    TableScalePreScaledValsNotSpecd = -200349,
    #[error("TableScaleScaledValsNotSpecd")]
    TableScaleScaledValsNotSpecd = -200348,
    #[error("IntermediateBufferSizeNotMultipleOfIncr")]
    IntermediateBufferSizeNotMultipleOfIncr = -200347,
    #[error("EventPulseWidthOutOfRange")]
    EventPulseWidthOutOfRange = -200346,
    #[error("EventDelayOutOfRange")]
    EventDelayOutOfRange = -200345,
    #[error("SampPerChanNotMultipleOfIncr")]
    SampPerChanNotMultipleOfIncr = -200344,
    #[error("CannotCalculateNumSampsTaskNotStarted")]
    CannotCalculateNumSampsTaskNotStarted = -200343,
    #[error("ScriptNotInMem")]
    ScriptNotInMem = -200342,
    #[error("OnboardMemTooSmall")]
    OnboardMemTooSmall = -200341,
    #[error("ReadAllAvailableDataWithoutBuffer")]
    ReadAllAvailableDataWithoutBuffer = -200340,
    #[error("PulseActiveAtStart")]
    PulseActiveAtStart = -200339,
    #[error("CalTempNotSupported")]
    CalTempNotSupported = -200338,
    #[error("DelayFromSampClkTooLong")]
    DelayFromSampClkTooLong = -200337,
    #[error("DelayFromSampClkTooShort")]
    DelayFromSampClkTooShort = -200336,
    #[error("AIConvRateTooHigh")]
    AIConvRateTooHigh = -200335,
    #[error("DelayFromStartTrigTooLong")]
    DelayFromStartTrigTooLong = -200334,
    #[error("DelayFromStartTrigTooShort")]
    DelayFromStartTrigTooShort = -200333,
    #[error("SampRateTooHigh")]
    SampRateTooHigh = -200332,
    #[error("SampRateTooLow")]
    SampRateTooLow = -200331,
    #[error("PFI0UsedForAnalogAndDigitalSrc")]
    PFI0UsedForAnalogAndDigitalSrc = -200330,
    #[error("PrimingCfgFIFO")]
    PrimingCfgFIFO = -200329,
    #[error("CannotOpenTopologyCfgFile")]
    CannotOpenTopologyCfgFile = -200328,
    #[error("InvalidDTInsideWfmDataType")]
    InvalidDTInsideWfmDataType = -200327,
    #[error("RouteSrcAndDestSame")]
    RouteSrcAndDestSame = -200326,
    #[error("ReversePolynomialCoefNotSpecd")]
    ReversePolynomialCoefNotSpecd = -200325,
    #[error("DevAbsentOrUnavailable")]
    DevAbsentOrUnavailable = -200324,
    #[error("NoAdvTrigForMultiDevScan")]
    NoAdvTrigForMultiDevScan = -200323,
    #[error("InterruptsInsufficientDataXferMech")]
    InterruptsInsufficientDataXferMech = -200322,
    #[error("InvalidAttentuationBasedOnMinMax")]
    InvalidAttentuationBasedOnMinMax = -200321,
    #[error("CabledModuleCannotRouteSSH")]
    CabledModuleCannotRouteSSH = -200320,
    #[error("CabledModuleCannotRouteConvClk")]
    CabledModuleCannotRouteConvClk = -200319,
    #[error("InvalidExcitValForScaling")]
    InvalidExcitValForScaling = -200318,
    #[error("NoDevMemForScript")]
    NoDevMemForScript = -200317,
    #[error("ScriptDataUnderflow")]
    ScriptDataUnderflow = -200316,
    #[error("NoDevMemForWaveform")]
    NoDevMemForWaveform = -200315,
    #[error("StreamDCMBecameUnlocked")]
    StreamDCMBecameUnlocked = -200314,
    #[error("StreamDCMLock")]
    StreamDCMLock = -200313,
    #[error("WaveformNotInMem")]
    WaveformNotInMem = -200312,
    #[error("WaveformWriteOutOfBounds")]
    WaveformWriteOutOfBounds = -200311,
    #[error("WaveformPreviouslyAllocated")]
    WaveformPreviouslyAllocated = -200310,
    #[error("SampClkTbMasterTbDivNotAppropriateForSampTbSrc")]
    SampClkTbMasterTbDivNotAppropriateForSampTbSrc = -200309,
    #[error("SampTbRateSampTbSrcMismatch")]
    SampTbRateSampTbSrcMismatch = -200308,
    #[error("MasterTbRateMasterTbSrcMismatch")]
    MasterTbRateMasterTbSrcMismatch = -200307,
    #[error("SampsPerChanTooBig")]
    SampsPerChanTooBig = -200306,
    #[error("FinitePulseTrainNotPossible")]
    FinitePulseTrainNotPossible = -200305,
    #[error("ExtMasterTimebaseRateNotSpecified")]
    ExtMasterTimebaseRateNotSpecified = -200304,
    #[error("ExtSampClkSrcNotSpecified")]
    ExtSampClkSrcNotSpecified = -200303,
    #[error("InputSignalSlowerThanMeasTime")]
    InputSignalSlowerThanMeasTime = -200302,
    #[error("CannotUpdatePulseGenProperty")]
    CannotUpdatePulseGenProperty = -200301,
    #[error("InvalidTimingType")]
    InvalidTimingType = -200300,
    #[error("PropertyUnavailWhenUsingOnboardMemory")]
    PropertyUnavailWhenUsingOnboardMemory = -200297,
    #[error("CannotWriteAfterStartWithOnboardMemory")]
    CannotWriteAfterStartWithOnboardMemory = -200295,
    #[error("NotEnoughSampsWrittenForInitialXferRqstCondition")]
    NotEnoughSampsWrittenForInitialXferRqstCondition = -200294,
    #[error("NoMoreSpace")]
    NoMoreSpace = -200293,
    #[error("SamplesCanNotYetBeWritten")]
    SamplesCanNotYetBeWritten = -200292,
    #[error("GenStoppedToPreventIntermediateBufferRegenOfOldSamples")]
    GenStoppedToPreventIntermediateBufferRegenOfOldSamples = -200291,
    #[error("GenStoppedToPreventRegenOfOldSamples")]
    GenStoppedToPreventRegenOfOldSamples = -200290,
    #[error("SamplesNoLongerWriteable")]
    SamplesNoLongerWriteable = -200289,
    #[error("SamplesWillNeverBeGenerated")]
    SamplesWillNeverBeGenerated = -200288,
    #[error("NegativeWriteSampleNumber")]
    NegativeWriteSampleNumber = -200287,
    #[error("NoAcqStarted")]
    NoAcqStarted = -200286,
    #[error("SamplesNotYetAvailable")]
    SamplesNotYetAvailable = -200284,
    #[error("AcqStoppedToPreventIntermediateBufferOverflow")]
    AcqStoppedToPreventIntermediateBufferOverflow = -200283,
    #[error("NoRefTrigConfigured")]
    NoRefTrigConfigured = -200282,
    #[error("CannotReadRelativeToRefTrigUntilDone")]
    CannotReadRelativeToRefTrigUntilDone = -200281,
    #[error("SamplesNoLongerAvailable")]
    SamplesNoLongerAvailable = -200279,
    #[error("SamplesWillNeverBeAvailable")]
    SamplesWillNeverBeAvailable = -200278,
    #[error("NegativeReadSampleNumber")]
    NegativeReadSampleNumber = -200277,
    #[error("ExternalSampClkAndRefClkThruSameTerm")]
    ExternalSampClkAndRefClkThruSameTerm = -200276,
    #[error("ExtSampClkRateTooLowForClkIn")]
    ExtSampClkRateTooLowForClkIn = -200275,
    #[error("ExtSampClkRateTooHighForBackplane")]
    ExtSampClkRateTooHighForBackplane = -200274,
    #[error("SampClkRateAndDivCombo")]
    SampClkRateAndDivCombo = -200273,
    #[error("SampClkRateTooLowForDivDown")]
    SampClkRateTooLowForDivDown = -200272,
    #[error("ProductOfAOMinAndGainTooSmall")]
    ProductOfAOMinAndGainTooSmall = -200271,
    #[error("InterpolationRateNotPossible")]
    InterpolationRateNotPossible = -200270,
    #[error("OffsetTooLarge")]
    OffsetTooLarge = -200269,
    #[error("OffsetTooSmall")]
    OffsetTooSmall = -200268,
    #[error("ProductOfAOMaxAndGainTooLarge")]
    ProductOfAOMaxAndGainTooLarge = -200267,
    #[error("MinAndMaxNotSymmetric")]
    MinAndMaxNotSymmetric = -200266,
    #[error("InvalidAnalogTrigSrc")]
    InvalidAnalogTrigSrc = -200265,
    #[error("TooManyChansForAnalogRefTrig")]
    TooManyChansForAnalogRefTrig = -200264,
    #[error("TooManyChansForAnalogPauseTrig")]
    TooManyChansForAnalogPauseTrig = -200263,
    #[error("TrigWhenOnDemandSampTiming")]
    TrigWhenOnDemandSampTiming = -200262,
    #[error("InconsistentAnalogTrigSettings")]
    InconsistentAnalogTrigSettings = -200261,
    #[error("MemMapDataXferModeSampTimingCombo")]
    MemMapDataXferModeSampTimingCombo = -200260,
    #[error("InvalidJumperedAttr")]
    InvalidJumperedAttr = -200259,
    #[error("InvalidGainBasedOnMinMax")]
    InvalidGainBasedOnMinMax = -200258,
    #[error("InconsistentExcit")]
    InconsistentExcit = -200257,
    #[error("TopologyNotSupportedByCfgTermBlock")]
    TopologyNotSupportedByCfgTermBlock = -200256,
    #[error("BuiltInTempSensorNotSupported")]
    BuiltInTempSensorNotSupported = -200255,
    #[error("InvalidTerm")]
    InvalidTerm = -200254,
    #[error("CannotTristateTerm")]
    CannotTristateTerm = -200253,
    #[error("CannotTristateBusyTerm")]
    CannotTristateBusyTerm = -200252,
    #[error("NoDMAChansAvailable")]
    NoDMAChansAvailable = -200251,
    #[error("InvalidWaveformLengthWithinLoopInScript")]
    InvalidWaveformLengthWithinLoopInScript = -200250,
    #[error("InvalidSubsetLengthWithinLoopInScript")]
    InvalidSubsetLengthWithinLoopInScript = -200249,
    #[error("MarkerPosInvalidForLoopInScript")]
    MarkerPosInvalidForLoopInScript = -200248,
    #[error("IntegerExpectedInScript")]
    IntegerExpectedInScript = -200247,
    #[error("PLLBecameUnlocked")]
    PLLBecameUnlocked = -200246,
    #[error("PLLLock")]
    PLLLock = -200245,
    #[error("DDCClkOutDCMBecameUnlocked")]
    DDCClkOutDCMBecameUnlocked = -200244,
    #[error("DDCClkOutDCMLock")]
    DDCClkOutDCMLock = -200243,
    #[error("ClkDoublerDCMBecameUnlocked")]
    ClkDoublerDCMBecameUnlocked = -200242,
    #[error("ClkDoublerDCMLock")]
    ClkDoublerDCMLock = -200241,
    #[error("SampClkDCMBecameUnlocked")]
    SampClkDCMBecameUnlocked = -200240,
    #[error("SampClkDCMLock")]
    SampClkDCMLock = -200239,
    #[error("SampClkTimebaseDCMBecameUnlocked")]
    SampClkTimebaseDCMBecameUnlocked = -200238,
    #[error("SampClkTimebaseDCMLock")]
    SampClkTimebaseDCMLock = -200237,
    #[error("AttrCannotBeReset")]
    AttrCannotBeReset = -200236,
    #[error("ExplanationNotFound")]
    ExplanationNotFound = -200235,
    #[error("WriteBufferTooSmall")]
    WriteBufferTooSmall = -200234,
    #[error("SpecifiedAttrNotValid")]
    SpecifiedAttrNotValid = -200233,
    #[error("AttrCannotBeRead")]
    AttrCannotBeRead = -200232,
    #[error("AttrCannotBeSet")]
    AttrCannotBeSet = -200231,
    #[error("NULLPtrForCApi")]
    NULLPtrForCApi = -200230,
    #[error("ReadBufferTooSmall")]
    ReadBufferTooSmall = -200229,
    #[error("BufferTooSmallForString")]
    BufferTooSmallForString = -200228,
    #[error("NoAvailTrigLinesOnDevice")]
    NoAvailTrigLinesOnDevice = -200227,
    #[error("TrigBusLineNotAvail")]
    TrigBusLineNotAvail = -200226,
    #[error("CouldNotReserveRequestedTrigLine")]
    CouldNotReserveRequestedTrigLine = -200225,
    #[error("TrigLineNotFound")]
    TrigLineNotFound = -200224,
    #[error("SCXI1126ThreshHystCombination")]
    SCXI1126ThreshHystCombination = -200223,
    #[error("AcqStoppedToPreventInputBufferOverwrite")]
    AcqStoppedToPreventInputBufferOverwrite = -200222,
    #[error("TimeoutExceeded")]
    TimeoutExceeded = -200221,
    #[error("InvalidDeviceID")]
    InvalidDeviceID = -200220,
    #[error("InvalidAOChanOrder")]
    InvalidAOChanOrder = -200219,
    #[error("SampleTimingTypeAndDataXferMode")]
    SampleTimingTypeAndDataXferMode = -200218,
    #[error("BufferWithOnDemandSampTiming")]
    BufferWithOnDemandSampTiming = -200217,
    #[error("BufferAndDataXferMode")]
    BufferAndDataXferMode = -200216,
    #[error("MemMapAndBuffer")]
    MemMapAndBuffer = -200215,
    #[error("NoAnalogTrigHW")]
    NoAnalogTrigHW = -200214,
    #[error("TooManyPretrigPlusMinPostTrigSamps")]
    TooManyPretrigPlusMinPostTrigSamps = -200213,
    #[error("InconsistentUnitsSpecified")]
    InconsistentUnitsSpecified = -200212,
    #[error("MultipleRelaysForSingleRelayOp")]
    MultipleRelaysForSingleRelayOp = -200211,
    #[error("MultipleDevIDsPerChassisSpecifiedInList")]
    MultipleDevIDsPerChassisSpecifiedInList = -200210,
    #[error("DuplicateDevIDInList")]
    DuplicateDevIDInList = -200209,
    #[error("InvalidRangeStatementCharInList")]
    InvalidRangeStatementCharInList = -200208,
    #[error("InvalidDeviceIDInList")]
    InvalidDeviceIDInList = -200207,
    #[error("TriggerPolarityConflict")]
    TriggerPolarityConflict = -200206,
    #[error("CannotScanWithCurrentTopology")]
    CannotScanWithCurrentTopology = -200205,
    #[error("UnexpectedIdentifierInFullySpecifiedPathInList")]
    UnexpectedIdentifierInFullySpecifiedPathInList = -200204,
    #[error("SwitchCannotDriveMultipleTrigLines")]
    SwitchCannotDriveMultipleTrigLines = -200203,
    #[error("InvalidRelayName")]
    InvalidRelayName = -200202,
    #[error("SwitchScanlistTooBig")]
    SwitchScanlistTooBig = -200201,
    #[error("SwitchChanInUse")]
    SwitchChanInUse = -200200,
    #[error("SwitchNotResetBeforeScan")]
    SwitchNotResetBeforeScan = -200199,
    #[error("InvalidTopology")]
    InvalidTopology = -200198,
    #[error("AttrNotSupported")]
    AttrNotSupported = -200197,
    #[error("UnexpectedEndOfActionsInList")]
    UnexpectedEndOfActionsInList = -200196,
    #[error("PowerLimitExceeded")]
    PowerLimitExceeded = -200195,
    #[error("HWUnexpectedlyPoweredOffAndOn")]
    HWUnexpectedlyPoweredOffAndOn = -200194,
    #[error("SwitchOperationNotSupported")]
    SwitchOperationNotSupported = -200193,
    #[error("OnlyContinuousScanSupported")]
    OnlyContinuousScanSupported = -200192,
    #[error("SwitchDifferentTopologyWhenScanning")]
    SwitchDifferentTopologyWhenScanning = -200191,
    #[error("DisconnectPathNotSameAsExistingPath")]
    DisconnectPathNotSameAsExistingPath = -200190,
    #[error("ConnectionNotPermittedOnChanReservedForRouting")]
    ConnectionNotPermittedOnChanReservedForRouting = -200189,
    #[error("CannotConnectSrcChans")]
    CannotConnectSrcChans = -200188,
    #[error("CannotConnectChannelToItself")]
    CannotConnectChannelToItself = -200187,
    #[error("ChannelNotReservedForRouting")]
    ChannelNotReservedForRouting = -200186,
    #[error("CannotConnectChansDirectly")]
    CannotConnectChansDirectly = -200185,
    #[error("ChansAlreadyConnected")]
    ChansAlreadyConnected = -200184,
    #[error("ChanDuplicatedInPath")]
    ChanDuplicatedInPath = -200183,
    #[error("NoPathToDisconnect")]
    NoPathToDisconnect = -200182,
    #[error("InvalidSwitchChan")]
    InvalidSwitchChan = -200181,
    #[error("NoPathAvailableBetween2SwitchChans")]
    NoPathAvailableBetween2SwitchChans = -200180,
    #[error("ExplicitConnectionExists")]
    ExplicitConnectionExists = -200179,
    #[error("SwitchDifferentSettlingTimeWhenScanning")]
    SwitchDifferentSettlingTimeWhenScanning = -200178,
    #[error("OperationOnlyPermittedWhileScanning")]
    OperationOnlyPermittedWhileScanning = -200177,
    #[error("OperationNotPermittedWhileScanning")]
    OperationNotPermittedWhileScanning = -200176,
    #[error("HardwareNotResponding")]
    HardwareNotResponding = -200175,
    #[error("InvalidSampAndMasterTimebaseRateCombo")]
    InvalidSampAndMasterTimebaseRateCombo = -200173,
    #[error("NonZeroBufferSizeInProgIOXfer")]
    NonZeroBufferSizeInProgIOXfer = -200172,
    #[error("VirtualChanNameUsed")]
    VirtualChanNameUsed = -200171,
    #[error("PhysicalChanDoesNotExist")]
    PhysicalChanDoesNotExist = -200170,
    #[error("MemMapOnlyForProgIOXfer")]
    MemMapOnlyForProgIOXfer = -200169,
    #[error("TooManyChans")]
    TooManyChans = -200168,
    #[error("CannotHaveCJTempWithOtherChans")]
    CannotHaveCJTempWithOtherChans = -200167,
    #[error("OutputBufferUnderwrite")]
    OutputBufferUnderwrite = -200166,
    #[error("SensorInvalidCompletionResistance")]
    SensorInvalidCompletionResistance = -200163,
    #[error("VoltageExcitIncompatibleWith2WireCfg")]
    VoltageExcitIncompatibleWith2WireCfg = -200162,
    #[error("IntExcitSrcNotAvailable")]
    IntExcitSrcNotAvailable = -200161,
    #[error("CannotCreateChannelAfterTaskVerified")]
    CannotCreateChannelAfterTaskVerified = -200160,
    #[error("LinesReservedForSCXIControl")]
    LinesReservedForSCXIControl = -200159,
    #[error("CouldNotReserveLinesForSCXIControl")]
    CouldNotReserveLinesForSCXIControl = -200158,
    #[error("CalibrationFailed")]
    CalibrationFailed = -200157,
    #[error("ReferenceFrequencyInvalid")]
    ReferenceFrequencyInvalid = -200156,
    #[error("ReferenceResistanceInvalid")]
    ReferenceResistanceInvalid = -200155,
    #[error("ReferenceCurrentInvalid")]
    ReferenceCurrentInvalid = -200154,
    #[error("ReferenceVoltageInvalid")]
    ReferenceVoltageInvalid = -200153,
    #[error("EEPROMDataInvalid")]
    EEPROMDataInvalid = -200152,
    #[error("CabledModuleNotCapableOfRoutingAI")]
    CabledModuleNotCapableOfRoutingAI = -200151,
    #[error("ChannelNotAvailableInParallelMode")]
    ChannelNotAvailableInParallelMode = -200150,
    #[error("ExternalTimebaseRateNotKnownForDelay")]
    ExternalTimebaseRateNotKnownForDelay = -200149,
    #[error("FREQOUTCannotProduceDesiredFrequency")]
    FREQOUTCannotProduceDesiredFrequency = -200148,
    #[error("MultipleCounterInputTask")]
    MultipleCounterInputTask = -200147,
    #[error("CounterStartPauseTriggerConflict")]
    CounterStartPauseTriggerConflict = -200146,
    #[error("CounterInputPauseTriggerAndSampleClockInvalid")]
    CounterInputPauseTriggerAndSampleClockInvalid = -200145,
    #[error("CounterOutputPauseTriggerInvalid")]
    CounterOutputPauseTriggerInvalid = -200144,
    #[error("CounterTimebaseRateNotSpecified")]
    CounterTimebaseRateNotSpecified = -200143,
    #[error("CounterTimebaseRateNotFound")]
    CounterTimebaseRateNotFound = -200142,
    #[error("CounterOverflow")]
    CounterOverflow = -200141,
    #[error("CounterNoTimebaseEdgesBetweenGates")]
    CounterNoTimebaseEdgesBetweenGates = -200140,
    #[error("CounterMaxMinRangeFreq")]
    CounterMaxMinRangeFreq = -200139,
    #[error("CounterMaxMinRangeTime")]
    CounterMaxMinRangeTime = -200138,
    #[error("SuitableTimebaseNotFoundTimeCombo")]
    SuitableTimebaseNotFoundTimeCombo = -200137,
    #[error("SuitableTimebaseNotFoundFrequencyCombo")]
    SuitableTimebaseNotFoundFrequencyCombo = -200136,
    #[error("InternalTimebaseSourceDivisorCombo")]
    InternalTimebaseSourceDivisorCombo = -200135,
    #[error("InternalTimebaseSourceRateCombo")]
    InternalTimebaseSourceRateCombo = -200134,
    #[error("InternalTimebaseRateDivisorSourceCombo")]
    InternalTimebaseRateDivisorSourceCombo = -200133,
    #[error("ExternalTimebaseRateNotknownForRate")]
    ExternalTimebaseRateNotknownForRate = -200132,
    #[error("AnalogTrigChanNotFirstInScanList")]
    AnalogTrigChanNotFirstInScanList = -200131,
    #[error("NoDivisorForExternalSignal")]
    NoDivisorForExternalSignal = -200130,
    #[error("AttributeInconsistentAcrossRepeatedPhysicalChannels")]
    AttributeInconsistentAcrossRepeatedPhysicalChannels = -200128,
    #[error("CannotHandshakeWithPort0")]
    CannotHandshakeWithPort0 = -200127,
    #[error("ControlLineConflictOnPortC")]
    ControlLineConflictOnPortC = -200126,
    #[error("Lines4To7ConfiguredForOutput")]
    Lines4To7ConfiguredForOutput = -200125,
    #[error("Lines4To7ConfiguredForInput")]
    Lines4To7ConfiguredForInput = -200124,
    #[error("Lines0To3ConfiguredForOutput")]
    Lines0To3ConfiguredForOutput = -200123,
    #[error("Lines0To3ConfiguredForInput")]
    Lines0To3ConfiguredForInput = -200122,
    #[error("PortConfiguredForOutput")]
    PortConfiguredForOutput = -200121,
    #[error("PortConfiguredForInput")]
    PortConfiguredForInput = -200120,
    #[error("PortConfiguredForStaticDigitalOps")]
    PortConfiguredForStaticDigitalOps = -200119,
    #[error("PortReservedForHandshaking")]
    PortReservedForHandshaking = -200118,
    #[error("PortDoesNotSupportHandshakingDataIO")]
    PortDoesNotSupportHandshakingDataIO = -200117,
    #[error("CannotTristate8255OutputLines")]
    CannotTristate8255OutputLines = -200116,
    #[error("TemperatureOutOfRangeForCalibration")]
    TemperatureOutOfRangeForCalibration = -200113,
    #[error("CalibrationHandleInvalid")]
    CalibrationHandleInvalid = -200112,
    #[error("PasswordRequired")]
    PasswordRequired = -200111,
    #[error("IncorrectPassword")]
    IncorrectPassword = -200110,
    #[error("PasswordTooLong")]
    PasswordTooLong = -200109,
    #[error("CalibrationSessionAlreadyOpen")]
    CalibrationSessionAlreadyOpen = -200108,
    #[error("SCXIModuleIncorrect")]
    SCXIModuleIncorrect = -200107,
    #[error("AttributeInconsistentAcrossChannelsOnDevice")]
    AttributeInconsistentAcrossChannelsOnDevice = -200106,
    #[error("SCXI1122ResistanceChanNotSupportedForCfg")]
    SCXI1122ResistanceChanNotSupportedForCfg = -200105,
    #[error("BracketPairingMismatchInList")]
    BracketPairingMismatchInList = -200104,
    #[error("InconsistentNumSamplesToWrite")]
    InconsistentNumSamplesToWrite = -200103,
    #[error("IncorrectDigitalPattern")]
    IncorrectDigitalPattern = -200102,
    #[error("IncorrectNumChannelsToWrite")]
    IncorrectNumChannelsToWrite = -200101,
    #[error("IncorrectReadFunction")]
    IncorrectReadFunction = -200100,
    #[error("PhysicalChannelNotSpecified")]
    PhysicalChannelNotSpecified = -200099,
    #[error("MoreThanOneTerminal")]
    MoreThanOneTerminal = -200098,
    #[error("MoreThanOneActiveChannelSpecified")]
    MoreThanOneActiveChannelSpecified = -200097,
    #[error("InvalidNumberSamplesToRead")]
    InvalidNumberSamplesToRead = -200096,
    #[error("AnalogWaveformExpected")]
    AnalogWaveformExpected = -200095,
    #[error("DigitalWaveformExpected")]
    DigitalWaveformExpected = -200094,
    #[error("ActiveChannelNotSpecified")]
    ActiveChannelNotSpecified = -200093,
    #[error("FunctionNotSupportedForDeviceTasks")]
    FunctionNotSupportedForDeviceTasks = -200092,
    #[error("FunctionNotInLibrary")]
    FunctionNotInLibrary = -200091,
    #[error("LibraryNotPresent")]
    LibraryNotPresent = -200090,
    #[error("DuplicateTask")]
    DuplicateTask = -200089,
    #[error("InvalidTask")]
    InvalidTask = -200088,
    #[error("InvalidChannel")]
    InvalidChannel = -200087,
    #[error("InvalidSyntaxForPhysicalChannelRange")]
    InvalidSyntaxForPhysicalChannelRange = -200086,
    #[error("MinNotLessThanMax")]
    MinNotLessThanMax = -200082,
    #[error("SampleRateNumChansConvertPeriodCombo")]
    SampleRateNumChansConvertPeriodCombo = -200081,
    #[error("AODuringCounter1DMAConflict")]
    AODuringCounter1DMAConflict = -200079,
    #[error("AIDuringCounter0DMAConflict")]
    AIDuringCounter0DMAConflict = -200078,
    #[error("InvalidAttributeValue")]
    InvalidAttributeValue = -200077,
    #[error("SuppliedCurrentDataOutsideSpecifiedRange")]
    SuppliedCurrentDataOutsideSpecifiedRange = -200076,
    #[error("SuppliedVoltageDataOutsideSpecifiedRange")]
    SuppliedVoltageDataOutsideSpecifiedRange = -200075,
    #[error("CannotStoreCalConst")]
    CannotStoreCalConst = -200074,
    #[error("SCXIModuleNotFound")]
    SCXIModuleNotFound = -200073,
    #[error("DuplicatePhysicalChansNotSupported")]
    DuplicatePhysicalChansNotSupported = -200072,
    #[error("TooManyPhysicalChansInList")]
    TooManyPhysicalChansInList = -200071,
    #[error("InvalidAdvanceEventTriggerType")]
    InvalidAdvanceEventTriggerType = -200070,
    #[error("DeviceIsNotAValidSwitch")]
    DeviceIsNotAValidSwitch = -200069,
    #[error("DeviceDoesNotSupportScanning")]
    DeviceDoesNotSupportScanning = -200068,
    #[error("ScanListCannotBeTimed")]
    ScanListCannotBeTimed = -200067,
    #[error("ConnectOperatorInvalidAtPointInList")]
    ConnectOperatorInvalidAtPointInList = -200066,
    #[error("UnexpectedSwitchActionInList")]
    UnexpectedSwitchActionInList = -200065,
    #[error("UnexpectedSeparatorInList")]
    UnexpectedSeparatorInList = -200064,
    #[error("ExpectedTerminatorInList")]
    ExpectedTerminatorInList = -200063,
    #[error("ExpectedConnectOperatorInList")]
    ExpectedConnectOperatorInList = -200062,
    #[error("ExpectedSeparatorInList")]
    ExpectedSeparatorInList = -200061,
    #[error("FullySpecifiedPathInListContainsRange")]
    FullySpecifiedPathInListContainsRange = -200060,
    #[error("ConnectionSeparatorAtEndOfList")]
    ConnectionSeparatorAtEndOfList = -200059,
    #[error("IdentifierInListTooLong")]
    IdentifierInListTooLong = -200058,
    #[error("DuplicateDeviceIDInListWhenSettling")]
    DuplicateDeviceIDInListWhenSettling = -200057,
    #[error("ChannelNameNotSpecifiedInList")]
    ChannelNameNotSpecifiedInList = -200056,
    #[error("DeviceIDNotSpecifiedInList")]
    DeviceIDNotSpecifiedInList = -200055,
    #[error("SemicolonDoesNotFollowRangeInList")]
    SemicolonDoesNotFollowRangeInList = -200054,
    #[error("SwitchActionInListSpansMultipleDevices")]
    SwitchActionInListSpansMultipleDevices = -200053,
    #[error("RangeWithoutAConnectActionInList")]
    RangeWithoutAConnectActionInList = -200052,
    #[error("InvalidIdentifierFollowingSeparatorInList")]
    InvalidIdentifierFollowingSeparatorInList = -200051,
    #[error("InvalidChannelNameInList")]
    InvalidChannelNameInList = -200050,
    #[error("InvalidNumberInRepeatStatementInList")]
    InvalidNumberInRepeatStatementInList = -200049,
    #[error("InvalidTriggerLineInList")]
    InvalidTriggerLineInList = -200048,
    #[error("InvalidIdentifierInListFollowingDeviceID")]
    InvalidIdentifierInListFollowingDeviceID = -200047,
    #[error("InvalidIdentifierInListAtEndOfSwitchAction")]
    InvalidIdentifierInListAtEndOfSwitchAction = -200046,
    #[error("DeviceRemoved")]
    DeviceRemoved = -200045,
    #[error("RoutingPathNotAvailable")]
    RoutingPathNotAvailable = -200044,
    #[error("RoutingHardwareBusy")]
    RoutingHardwareBusy = -200043,
    #[error("RequestedSignalInversionForRoutingNotPossible")]
    RequestedSignalInversionForRoutingNotPossible = -200042,
    #[error("InvalidRoutingDestinationTerminalName")]
    InvalidRoutingDestinationTerminalName = -200041,
    #[error("InvalidRoutingSourceTerminalName")]
    InvalidRoutingSourceTerminalName = -200040,
    #[error("RoutingNotSupportedForDevice")]
    RoutingNotSupportedForDevice = -200039,
    #[error("WaitIsLastInstructionOfLoopInScript")]
    WaitIsLastInstructionOfLoopInScript = -200038,
    #[error("ClearIsLastInstructionOfLoopInScript")]
    ClearIsLastInstructionOfLoopInScript = -200037,
    #[error("InvalidLoopIterationsInScript")]
    InvalidLoopIterationsInScript = -200036,
    #[error("RepeatLoopNestingTooDeepInScript")]
    RepeatLoopNestingTooDeepInScript = -200035,
    #[error("MarkerPositionOutsideSubsetInScript")]
    MarkerPositionOutsideSubsetInScript = -200034,
    #[error("SubsetStartOffsetNotAlignedInScript")]
    SubsetStartOffsetNotAlignedInScript = -200033,
    #[error("InvalidSubsetLengthInScript")]
    InvalidSubsetLengthInScript = -200032,
    #[error("MarkerPositionNotAlignedInScript")]
    MarkerPositionNotAlignedInScript = -200031,
    #[error("SubsetOutsideWaveformInScript")]
    SubsetOutsideWaveformInScript = -200030,
    #[error("MarkerOutsideWaveformInScript")]
    MarkerOutsideWaveformInScript = -200029,
    #[error("WaveformInScriptNotInMem")]
    WaveformInScriptNotInMem = -200028,
    #[error("KeywordExpectedInScript")]
    KeywordExpectedInScript = -200027,
    #[error("BufferNameExpectedInScript")]
    BufferNameExpectedInScript = -200026,
    #[error("ProcedureNameExpectedInScript")]
    ProcedureNameExpectedInScript = -200025,
    #[error("ScriptHasInvalidIdentifier")]
    ScriptHasInvalidIdentifier = -200024,
    #[error("ScriptHasInvalidCharacter")]
    ScriptHasInvalidCharacter = -200023,
    #[error("ResourceAlreadyReserved")]
    ResourceAlreadyReserved = -200022,
    #[error("SelfTestFailed")]
    SelfTestFailed = -200020,
    #[error("ADCOverrun")]
    ADCOverrun = -200019,
    #[error("DACUnderflow")]
    DACUnderflow = -200018,
    #[error("InputFIFOUnderflow")]
    InputFIFOUnderflow = -200017,
    #[error("OutputFIFOUnderflow")]
    OutputFIFOUnderflow = -200016,
    #[error("SCXISerialCommunication")]
    SCXISerialCommunication = -200015,
    #[error("DigitalTerminalSpecifiedMoreThanOnce")]
    DigitalTerminalSpecifiedMoreThanOnce = -200014,
    #[error("DigitalOutputNotSupported")]
    DigitalOutputNotSupported = -200012,
    #[error("InconsistentChannelDirections")]
    InconsistentChannelDirections = -200011,
    #[error("InputFIFOOverflow")]
    InputFIFOOverflow = -200010,
    #[error("TimeStampOverwritten")]
    TimeStampOverwritten = -200009,
    #[error("StopTriggerHasNotOccurred")]
    StopTriggerHasNotOccurred = -200008,
    #[error("RecordNotAvailable")]
    RecordNotAvailable = -200007,
    #[error("RecordOverwritten")]
    RecordOverwritten = -200006,
    #[error("DataNotAvailable")]
    DataNotAvailable = -200005,
    #[error("DataOverwrittenInDeviceMemory")]
    DataOverwrittenInDeviceMemory = -200004,
    #[error("DuplicatedChannel")]
    DuplicatedChannel = -200003,
    #[error("InterfaceObsoletedRouting")]
    InterfaceObsoletedRouting = -89169,
    #[error("RoCoServiceNotAvailableRouting")]
    RoCoServiceNotAvailableRouting = -89168,
    #[error("RoutingDestTermPXIDStarXNotInSystemTimingSlotRouting")]
    RoutingDestTermPXIDStarXNotInSystemTimingSlotRouting = -89167,
    #[error("RoutingSrcTermPXIDStarXNotInSystemTimingSlotRouting")]
    RoutingSrcTermPXIDStarXNotInSystemTimingSlotRouting = -89166,
    #[error("RoutingSrcTermPXIDStarInNonDStarTriggerSlotRouting")]
    RoutingSrcTermPXIDStarInNonDStarTriggerSlotRouting = -89165,
    #[error("RoutingDestTermPXIDStarInNonDStarTriggerSlotRouting")]
    RoutingDestTermPXIDStarInNonDStarTriggerSlotRouting = -89164,
    #[error("RoutingDestTermPXIClk10InNotInStarTriggerSlotRouting")]
    RoutingDestTermPXIClk10InNotInStarTriggerSlotRouting = -89162,
    #[error("RoutingDestTermPXIClk10InNotInSystemTimingSlotRouting")]
    RoutingDestTermPXIClk10InNotInSystemTimingSlotRouting = -89161,
    #[error("RoutingDestTermPXIStarXNotInStarTriggerSlotRouting")]
    RoutingDestTermPXIStarXNotInStarTriggerSlotRouting = -89160,
    #[error("RoutingDestTermPXIStarXNotInSystemTimingSlotRouting")]
    RoutingDestTermPXIStarXNotInSystemTimingSlotRouting = -89159,
    #[error("RoutingSrcTermPXIStarXNotInStarTriggerSlotRouting")]
    RoutingSrcTermPXIStarXNotInStarTriggerSlotRouting = -89158,
    #[error("RoutingSrcTermPXIStarXNotInSystemTimingSlotRouting")]
    RoutingSrcTermPXIStarXNotInSystemTimingSlotRouting = -89157,
    #[error("RoutingSrcTermPXIStarInNonStarTriggerSlotRouting")]
    RoutingSrcTermPXIStarInNonStarTriggerSlotRouting = -89156,
    #[error("RoutingDestTermPXIStarInNonStarTriggerSlotRouting")]
    RoutingDestTermPXIStarInNonStarTriggerSlotRouting = -89155,
    #[error("RoutingDestTermPXIStarInStarTriggerSlotRouting")]
    RoutingDestTermPXIStarInStarTriggerSlotRouting = -89154,
    #[error("RoutingDestTermPXIStarInSystemTimingSlotRouting")]
    RoutingDestTermPXIStarInSystemTimingSlotRouting = -89153,
    #[error("RoutingSrcTermPXIStarInStarTriggerSlotRouting")]
    RoutingSrcTermPXIStarInStarTriggerSlotRouting = -89152,
    #[error("RoutingSrcTermPXIStarInSystemTimingSlotRouting")]
    RoutingSrcTermPXIStarInSystemTimingSlotRouting = -89151,
    #[error("InvalidSignalModifierRouting")]
    InvalidSignalModifierRouting = -89150,
    #[error("RoutingDestTermPXIClk10InNotInSlot2Routing")]
    RoutingDestTermPXIClk10InNotInSlot2Routing = -89149,
    #[error("RoutingDestTermPXIStarXNotInSlot2Routing")]
    RoutingDestTermPXIStarXNotInSlot2Routing = -89148,
    #[error("RoutingSrcTermPXIStarXNotInSlot2Routing")]
    RoutingSrcTermPXIStarXNotInSlot2Routing = -89147,
    #[error("RoutingSrcTermPXIStarInSlot16AndAboveRouting")]
    RoutingSrcTermPXIStarInSlot16AndAboveRouting = -89146,
    #[error("RoutingDestTermPXIStarInSlot16AndAboveRouting")]
    RoutingDestTermPXIStarInSlot16AndAboveRouting = -89145,
    #[error("RoutingDestTermPXIStarInSlot2Routing")]
    RoutingDestTermPXIStarInSlot2Routing = -89144,
    #[error("RoutingSrcTermPXIStarInSlot2Routing")]
    RoutingSrcTermPXIStarInSlot2Routing = -89143,
    #[error("RoutingDestTermPXIChassisNotIdentifiedRouting")]
    RoutingDestTermPXIChassisNotIdentifiedRouting = -89142,
    #[error("RoutingSrcTermPXIChassisNotIdentifiedRouting")]
    RoutingSrcTermPXIChassisNotIdentifiedRouting = -89141,
    #[error("TrigLineNotFoundSingleDevRouteRouting")]
    TrigLineNotFoundSingleDevRouteRouting = -89140,
    #[error("NoCommonTrigLineForRouteRouting")]
    NoCommonTrigLineForRouteRouting = -89139,
    #[error("ResourcesInUseForRouteInTaskRouting")]
    ResourcesInUseForRouteInTaskRouting = -89138,
    #[error("ResourcesInUseForRouteRouting")]
    ResourcesInUseForRouteRouting = -89137,
    #[error("RouteNotSupportedByHWRouting")]
    RouteNotSupportedByHWRouting = -89136,
    #[error("ResourcesInUseForInversionInTaskRouting")]
    ResourcesInUseForInversionInTaskRouting = -89135,
    #[error("ResourcesInUseForInversionRouting")]
    ResourcesInUseForInversionRouting = -89134,
    #[error("InversionNotSupportedByHWRouting")]
    InversionNotSupportedByHWRouting = -89133,
    #[error("ResourcesInUseForPropertyRouting")]
    ResourcesInUseForPropertyRouting = -89132,
    #[error("RouteSrcAndDestSameRouting")]
    RouteSrcAndDestSameRouting = -89131,
    #[error("DevAbsentOrUnavailableRouting")]
    DevAbsentOrUnavailableRouting = -89130,
    #[error("InvalidTermRouting")]
    InvalidTermRouting = -89129,
    #[error("CannotTristateTermRouting")]
    CannotTristateTermRouting = -89128,
    #[error("CannotTristateBusyTermRouting")]
    CannotTristateBusyTermRouting = -89127,
    #[error("CouldNotReserveRequestedTrigLineRouting")]
    CouldNotReserveRequestedTrigLineRouting = -89126,
    #[error("TrigLineNotFoundRouting")]
    TrigLineNotFoundRouting = -89125,
    #[error("RoutingPathNotAvailableRouting")]
    RoutingPathNotAvailableRouting = -89124,
    #[error("RoutingHardwareBusyRouting")]
    RoutingHardwareBusyRouting = -89123,
    #[error("RequestedSignalInversionForRoutingNotPossibleRouting")]
    RequestedSignalInversionForRoutingNotPossibleRouting = -89122,
    #[error("InvalidRoutingDestinationTerminalNameRouting")]
    InvalidRoutingDestinationTerminalNameRouting = -89121,
    #[error("InvalidRoutingSourceTerminalNameRouting")]
    InvalidRoutingSourceTerminalNameRouting = -89120,
    #[error("ServiceLocatorNotAvailableRouting")]
    ServiceLocatorNotAvailableRouting = -88907,
    #[error("CouldNotConnectToServerRouting")]
    CouldNotConnectToServerRouting = -88900,
    #[error("DeviceNameContainsSpacesOrPunctuationRouting")]
    DeviceNameContainsSpacesOrPunctuationRouting = -88720,
    #[error("DeviceNameContainsNonprintableCharactersRouting")]
    DeviceNameContainsNonprintableCharactersRouting = -88719,
    #[error("DeviceNameIsEmptyRouting")]
    DeviceNameIsEmptyRouting = -88718,
    #[error("DeviceNameNotFoundRouting")]
    DeviceNameNotFoundRouting = -88717,
    #[error("LocalRemoteDriverVersionMismatchRouting")]
    LocalRemoteDriverVersionMismatchRouting = -88716,
    #[error("DuplicateDeviceNameRouting")]
    DuplicateDeviceNameRouting = -88715,
    #[error("RuntimeAbortingRouting")]
    RuntimeAbortingRouting = -88710,
    #[error("RuntimeAbortedRouting")]
    RuntimeAbortedRouting = -88709,
    #[error("ResourceNotInPoolRouting")]
    ResourceNotInPoolRouting = -88708,
    #[error("DriverDeviceGUIDNotFoundRouting")]
    DriverDeviceGUIDNotFoundRouting = -88705,
    #[error("PALUSBTransactionError")]
    PALUSBTransactionError = -50808,
    #[error("PALIsocStreamBufferError")]
    PALIsocStreamBufferError = -50807,
    #[error("PALInvalidAddressComponent")]
    PALInvalidAddressComponent = -50806,
    #[error("PALSharingViolation")]
    PALSharingViolation = -50805,
    #[error("PALInvalidDeviceState")]
    PALInvalidDeviceState = -50804,
    #[error("PALConnectionReset")]
    PALConnectionReset = -50803,
    #[error("PALConnectionAborted")]
    PALConnectionAborted = -50802,
    #[error("PALConnectionRefused")]
    PALConnectionRefused = -50801,
    #[error("PALBusResetOccurred")]
    PALBusResetOccurred = -50800,
    #[error("PALWaitInterrupted")]
    PALWaitInterrupted = -50700,
    #[error("PALMessageUnderflow")]
    PALMessageUnderflow = -50651,
    #[error("PALMessageOverflow")]
    PALMessageOverflow = -50650,
    #[error("PALThreadAlreadyDead")]
    PALThreadAlreadyDead = -50604,
    #[error("PALThreadStackSizeNotSupported")]
    PALThreadStackSizeNotSupported = -50603,
    #[error("PALThreadControllerIsNotThreadCreator")]
    PALThreadControllerIsNotThreadCreator = -50602,
    #[error("PALThreadHasNoThreadObject")]
    PALThreadHasNoThreadObject = -50601,
    #[error("PALThreadCouldNotRun")]
    PALThreadCouldNotRun = -50600,
    #[error("PALSyncAbandoned")]
    PALSyncAbandoned = -50551,
    #[error("PALSyncTimedOut")]
    PALSyncTimedOut = -50550,
    #[error("PALReceiverSocketInvalid")]
    PALReceiverSocketInvalid = -50503,
    #[error("PALSocketListenerInvalid")]
    PALSocketListenerInvalid = -50502,
    #[error("PALSocketListenerAlreadyRegistered")]
    PALSocketListenerAlreadyRegistered = -50501,
    #[error("PALDispatcherAlreadyExported")]
    PALDispatcherAlreadyExported = -50500,
    #[error("PALDMALinkEventMissed")]
    PALDMALinkEventMissed = -50450,
    #[error("PALBusError")]
    PALBusError = -50413,
    #[error("PALRetryLimitExceeded")]
    PALRetryLimitExceeded = -50412,
    #[error("PALTransferOverread")]
    PALTransferOverread = -50411,
    #[error("PALTransferOverwritten")]
    PALTransferOverwritten = -50410,
    #[error("PALPhysicalBufferFull")]
    PALPhysicalBufferFull = -50409,
    #[error("PALPhysicalBufferEmpty")]
    PALPhysicalBufferEmpty = -50408,
    #[error("PALLogicalBufferFull")]
    PALLogicalBufferFull = -50407,
    #[error("PALLogicalBufferEmpty")]
    PALLogicalBufferEmpty = -50406,
    #[error("PALTransferAborted")]
    PALTransferAborted = -50405,
    #[error("PALTransferStopped")]
    PALTransferStopped = -50404,
    #[error("PALTransferInProgress")]
    PALTransferInProgress = -50403,
    #[error("PALTransferNotInProgress")]
    PALTransferNotInProgress = -50402,
    #[error("PALCommunicationsFault")]
    PALCommunicationsFault = -50401,
    #[error("PALTransferTimedOut")]
    PALTransferTimedOut = -50400,
    #[error("PALMemoryHeapNotEmpty")]
    PALMemoryHeapNotEmpty = -50355,
    #[error("PALMemoryBlockCheckFailed")]
    PALMemoryBlockCheckFailed = -50354,
    #[error("PALMemoryPageLockFailed")]
    PALMemoryPageLockFailed = -50353,
    #[error("PALMemoryFull")]
    PALMemoryFull = -50352,
    #[error("PALMemoryAlignmentFault")]
    PALMemoryAlignmentFault = -50351,
    #[error("PALMemoryConfigurationFault")]
    PALMemoryConfigurationFault = -50350,
    #[error("PALDeviceInitializationFault")]
    PALDeviceInitializationFault = -50303,
    #[error("PALDeviceNotSupported")]
    PALDeviceNotSupported = -50302,
    #[error("PALDeviceUnknown")]
    PALDeviceUnknown = -50301,
    #[error("PALDeviceNotFound")]
    PALDeviceNotFound = -50300,
    #[error("PALFeatureDisabled")]
    PALFeatureDisabled = -50265,
    #[error("PALComponentBusy")]
    PALComponentBusy = -50264,
    #[error("PALComponentAlreadyInstalled")]
    PALComponentAlreadyInstalled = -50263,
    #[error("PALComponentNotUnloadable")]
    PALComponentNotUnloadable = -50262,
    #[error("PALComponentNeverLoaded")]
    PALComponentNeverLoaded = -50261,
    #[error("PALComponentAlreadyLoaded")]
    PALComponentAlreadyLoaded = -50260,
    #[error("PALComponentCircularDependency")]
    PALComponentCircularDependency = -50259,
    #[error("PALComponentInitializationFault")]
    PALComponentInitializationFault = -50258,
    #[error("PALComponentImageCorrupt")]
    PALComponentImageCorrupt = -50257,
    #[error("PALFeatureNotSupported")]
    PALFeatureNotSupported = -50256,
    #[error("PALFunctionNotFound")]
    PALFunctionNotFound = -50255,
    #[error("PALFunctionObsolete")]
    PALFunctionObsolete = -50254,
    #[error("PALComponentTooNew")]
    PALComponentTooNew = -50253,
    #[error("PALComponentTooOld")]
    PALComponentTooOld = -50252,
    #[error("PALComponentNotFound")]
    PALComponentNotFound = -50251,
    #[error("PALVersionMismatch")]
    PALVersionMismatch = -50250,
    #[error("PALFileFault")]
    PALFileFault = -50209,
    #[error("PALFileWriteFault")]
    PALFileWriteFault = -50208,
    #[error("PALFileReadFault")]
    PALFileReadFault = -50207,
    #[error("PALFileSeekFault")]
    PALFileSeekFault = -50206,
    #[error("PALFileCloseFault")]
    PALFileCloseFault = -50205,
    #[error("PALFileOpenFault")]
    PALFileOpenFault = -50204,
    #[error("PALDiskFull")]
    PALDiskFull = -50203,
    #[error("PALOSFault")]
    PALOSFault = -50202,
    #[error("PALOSInitializationFault")]
    PALOSInitializationFault = -50201,
    #[error("PALOSUnsupported")]
    PALOSUnsupported = -50200,
    #[error("PALCalculationOverflow")]
    PALCalculationOverflow = -50175,
    #[error("PALHardwareFault")]
    PALHardwareFault = -50152,
    #[error("PALFirmwareFault")]
    PALFirmwareFault = -50151,
    #[error("PALSoftwareFault")]
    PALSoftwareFault = -50150,
    #[error("PALMessageQueueFull")]
    PALMessageQueueFull = -50108,
    #[error("PALResourceAmbiguous")]
    PALResourceAmbiguous = -50107,
    #[error("PALResourceBusy")]
    PALResourceBusy = -50106,
    #[error("PALResourceInitialized")]
    PALResourceInitialized = -50105,
    #[error("PALResourceNotInitialized")]
    PALResourceNotInitialized = -50104,
    #[error("PALResourceReserved")]
    PALResourceReserved = -50103,
    #[error("PALResourceNotReserved")]
    PALResourceNotReserved = -50102,
    #[error("PALResourceNotAvailable")]
    PALResourceNotAvailable = -50101,
    #[error("PALResourceOwnedBySystem")]
    PALResourceOwnedBySystem = -50100,
    #[error("PALBadToken")]
    PALBadToken = -50020,
    #[error("PALBadThreadMultitask")]
    PALBadThreadMultitask = -50019,
    #[error("PALBadLibrarySpecifier")]
    PALBadLibrarySpecifier = -50018,
    #[error("PALBadAddressSpace")]
    PALBadAddressSpace = -50017,
    #[error("PALBadWindowType")]
    PALBadWindowType = -50016,
    #[error("PALBadAddressClass")]
    PALBadAddressClass = -50015,
    #[error("PALBadWriteCount")]
    PALBadWriteCount = -50014,
    #[error("PALBadWriteOffset")]
    PALBadWriteOffset = -50013,
    #[error("PALBadWriteMode")]
    PALBadWriteMode = -50012,
    #[error("PALBadReadCount")]
    PALBadReadCount = -50011,
    #[error("PALBadReadOffset")]
    PALBadReadOffset = -50010,
    #[error("PALBadReadMode")]
    PALBadReadMode = -50009,
    #[error("PALBadCount")]
    PALBadCount = -50008,
    #[error("PALBadOffset")]
    PALBadOffset = -50007,
    #[error("PALBadMode")]
    PALBadMode = -50006,
    #[error("PALBadDataSize")]
    PALBadDataSize = -50005,
    #[error("PALBadPointer")]
    PALBadPointer = -50004,
    #[error("PALBadSelector")]
    PALBadSelector = -50003,
    #[error("PALBadDevice")]
    PALBadDevice = -50002,
    #[error("PALIrrelevantAttribute")]
    PALIrrelevantAttribute = -50001,
    #[error("PALValueConflict")]
    PALValueConflict = -50000,
}

#[derive(num_enum::TryFromPrimitive, Debug)]
#[repr(i32)]
pub enum Warning {
    TimestampCounterRolledOver = 200003,
    InputTerminationOverloaded = 200004,
    ADCOverloaded = 200005,
    PLLUnlocked = 200007,
    Counter0DMADuringAIConflict = 200008,
    Counter1DMADuringAOConflict = 200009,
    StoppedBeforeDone = 200010,
    RateViolatesSettlingTime = 200011,
    RateViolatesMaxADCRate = 200012,
    UserDefInfoStringTooLong = 200013,
    TooManyInterruptsPerSecond = 200014,
    PotentialGlitchDuringWrite = 200015,
    DevNotSelfCalibratedWithDAQmx = 200016,
    AISampRateTooLow = 200017,
    AIConvRateTooLow = 200018,
    ReadOffsetCoercion = 200019,
    PretrigCoercion = 200020,
    SampValCoercedToMax = 200021,
    SampValCoercedToMin = 200022,
    PropertyVersionNew = 200024,
    UserDefinedInfoTooLong = 200025,
    CAPIStringTruncatedToFitBuffer = 200026,
    SampClkRateTooLow = 200027,
    PossiblyInvalidCTRSampsInFiniteDMAAcq = 200028,
    RISAcqCompletedSomeBinsNotFilled = 200029,
    PXIDevTempExceedsMaxOpTemp = 200030,
    OutputGainTooLowForRFFreq = 200031,
    OutputGainTooHighForRFFreq = 200032,
    MultipleWritesBetweenSampClks = 200033,
    DeviceMayShutDownDueToHighTemp = 200034,
    RateViolatesMinADCRate = 200035,
    SampClkRateAboveDevSpecs = 200036,
    COPrevDAQmxWriteSettingsOverwrittenForHWTimedSinglePoint = 200037,
    LowpassFilterSettlingTimeExceedsUserTimeBetween2ADCConversions = 200038,
    LowpassFilterSettlingTimeExceedsDriverTimeBetween2ADCConversions = 200039,
    SampClkRateViolatesSettlingTimeForGen = 200040,
    InvalidCalConstValueForAI = 200041,
    InvalidCalConstValueForAO = 200042,
    ChanCalExpired = 200043,
    UnrecognizedEnumValueEncounteredInStorage = 200044,
    TableCRCNotCorrect = 200045,
    ExternalCRCNotCorrect = 200046,
    SelfCalCRCNotCorrect = 200047,
    DeviceSpecExceeded = 200048,
    OnlyGainCalibrated = 200049,
    ReversePowerProtectionActivated = 200050,
    OverVoltageProtectionActivated = 200051,
    BufferSizeNotMultipleOfSectorSize = 200052,
    SampleRateMayCauseAcqToFail = 200053,
    UserAreaCRCNotCorrect = 200054,
    PowerUpInfoCRCNotCorrect = 200055,
    ConnectionCountHasExceededRecommendedLimit = 200056,
    NetworkDeviceAlreadyAdded = 200057,
    AccessoryConnectionCountIsInvalid = 200058,
    UnableToDisconnectPorts = 200059,
    ReadRepeatedData = 200060,
    PXI5600NotConfigured = 200061,
    PXI5661IncorrectlyConfigured = 200062,
    PXIe5601NotConfigured = 200063,
    PXIe5663IncorrectlyConfigured = 200064,
    PXIe5663EIncorrectlyConfigured = 200065,
    PXIe5603NotConfigured = 200066,
    PXIe56655603IncorrectlyConfigured = 200067,
    PXIe56675603IncorrectlyConfigured = 200068,
    PXIe5605NotConfigured = 200069,
    PXIe56655605IncorrectlyConfigured = 200070,
    PXIe56675605IncorrectlyConfigured = 200071,
    PXIe5606NotConfigured = 200072,
    PXIe56655606IncorrectlyConfigured = 200073,
    PXI5610NotConfigured = 200074,
    PXI5610IncorrectlyConfigured = 200075,
    PXIe5611NotConfigured = 200076,
    PXIe5611IncorrectlyConfigured = 200077,
    USBHotfixForDAQ = 200078,
    NoChangeSupersededByIdleBehavior = 200079,
    ReadNotCompleteBeforeSampClk = 209800,
    WriteNotCompleteBeforeSampClk = 209801,
    WaitForNextSampClkDetectedMissedSampClk = 209802,
    OutputDataTransferConditionNotSupported = 209803,
    TimestampMayBeInvalid = 209804,
    FirstSampleTimestampInaccurate = 209805,
    PALValueConflict = 50000,
    PALIrrelevantAttribute = 50001,
    PALBadDevice = 50002,
    PALBadSelector = 50003,
    PALBadPointer = 50004,
    PALBadDataSize = 50005,
    PALBadMode = 50006,
    PALBadOffset = 50007,
    PALBadCount = 50008,
    PALBadReadMode = 50009,
    PALBadReadOffset = 50010,
    PALBadReadCount = 50011,
    PALBadWriteMode = 50012,
    PALBadWriteOffset = 50013,
    PALBadWriteCount = 50014,
    PALBadAddressClass = 50015,
    PALBadWindowType = 50016,
    PALBadThreadMultitask = 50019,
    PALResourceOwnedBySystem = 50100,
    PALResourceNotAvailable = 50101,
    PALResourceNotReserved = 50102,
    PALResourceReserved = 50103,
    PALResourceNotInitialized = 50104,
    PALResourceInitialized = 50105,
    PALResourceBusy = 50106,
    PALResourceAmbiguous = 50107,
    PALFirmwareFault = 50151,
    PALHardwareFault = 50152,
    PALOSUnsupported = 50200,
    PALOSFault = 50202,
    PALFunctionObsolete = 50254,
    PALFunctionNotFound = 50255,
    PALFeatureNotSupported = 50256,
    PALComponentInitializationFault = 50258,
    PALComponentAlreadyLoaded = 50260,
    PALComponentNotUnloadable = 50262,
    PALMemoryAlignmentFault = 50351,
    PALMemoryHeapNotEmpty = 50355,
    PALTransferNotInProgress = 50402,
    PALTransferInProgress = 50403,
    PALTransferStopped = 50404,
    PALTransferAborted = 50405,
    PALLogicalBufferEmpty = 50406,
    PALLogicalBufferFull = 50407,
    PALPhysicalBufferEmpty = 50408,
    PALPhysicalBufferFull = 50409,
    PALTransferOverwritten = 50410,
    PALTransferOverread = 50411,
    PALDispatcherAlreadyExported = 50500,
    PALSyncAbandoned = 50551,
}
