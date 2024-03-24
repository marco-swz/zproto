//! Settings in firmware version 7.05.

// #############################################################################
//  WARNING:
//  This file is generated by scripts/generate-apis. Do not modify it manually.
// #############################################################################

use crate::ascii::setting::Setting;
use crate::ascii::chain::scope::{AxisScope, DeviceScope};

define_settings! {

    /// The type representing the [`accel`](https://www.zaber.com/protocol-manual#topic_setting_accel) setting.
    pub struct Accel: Setting<Type = u32, Name = "accel">, AxisScope;
    /// The type representing the [`calibration.type`](https://www.zaber.com/protocol-manual#topic_setting_calibration_type) setting.
    pub struct CalibrationType: Setting<Type = u8, Name = "calibration.type">, AxisScope;
    /// The type representing the [`cloop.enable`](https://www.zaber.com/protocol-manual#topic_setting_cloop_enable) setting.
    pub struct CloopEnable: Setting<Type = bool, Name = "cloop.enable">, AxisScope;
    /// The type representing the [`cloop.recovery.enable`](https://www.zaber.com/protocol-manual#topic_setting_cloop_recovery_enable) setting.
    pub struct CloopRecoveryEnable: Setting<Type = bool, Name = "cloop.recovery.enable">, AxisScope;
    /// The type representing the [`cloop.settle.period`](https://www.zaber.com/protocol-manual#topic_setting_cloop_settle_period) setting.
    pub struct CloopSettlePeriod: Setting<Type = u32, Name = "cloop.settle.period">, AxisScope;
    /// The type representing the [`cloop.settle.tolerance`](https://www.zaber.com/protocol-manual#topic_setting_cloop_settle_tolerance) setting.
    pub struct CloopSettleTolerance: Setting<Type = u32, Name = "cloop.settle.tolerance">, AxisScope;
    /// The type representing the [`cloop.timeout`](https://www.zaber.com/protocol-manual#topic_setting_cloop_timeout) setting.
    pub struct CloopTimeout: Setting<Type = u16, Name = "cloop.timeout">, AxisScope;
    /// The type representing the [`comm.address`](https://www.zaber.com/protocol-manual#topic_setting_comm_address) setting.
    pub struct CommAddress: Setting<Type = u8, Name = "comm.address">, DeviceScope;
    /// The type representing the [`comm.alert`](https://www.zaber.com/protocol-manual#topic_setting_comm_alert) setting.
    pub struct CommAlert: Setting<Type = bool, Name = "comm.alert">, DeviceScope;
    /// The type representing the [`comm.checksum`](https://www.zaber.com/protocol-manual#topic_setting_comm_checksum) setting.
    pub struct CommChecksum: Setting<Type = bool, Name = "comm.checksum">, DeviceScope;
    /// The type representing the [`comm.rs232.baud`](https://www.zaber.com/protocol-manual#topic_setting_comm_rs232_baud) setting.
    pub struct CommRs232Baud: Setting<Type = u32, Name = "comm.rs232.baud">, DeviceScope;
    /// The type representing the [`device.id`](https://www.zaber.com/protocol-manual#topic_setting_device_id) setting.
    pub struct DeviceId: Setting<Type = u32, Name = "device.id">, DeviceScope;
    /// The type representing the [`driver.current.hold`](https://www.zaber.com/protocol-manual#topic_setting_driver_current_hold) setting.
    pub struct DriverCurrentHold: Setting<Type = u32, Name = "driver.current.hold">, AxisScope;
    /// The type representing the [`driver.current.max`](https://www.zaber.com/protocol-manual#topic_setting_driver_current_max) setting.
    pub struct DriverCurrentMax: Setting<Type = u32, Name = "driver.current.max">, AxisScope;
    /// The type representing the [`driver.current.run`](https://www.zaber.com/protocol-manual#topic_setting_driver_current_run) setting.
    pub struct DriverCurrentRun: Setting<Type = u32, Name = "driver.current.run">, AxisScope;
    /// The type representing the [`driver.current.servo`](https://www.zaber.com/protocol-manual#topic_setting_driver_current_servo) setting.
    pub struct DriverCurrentServo: Setting<Type = u32, Name = "driver.current.servo">, AxisScope;
    /// The type representing the [`driver.enabled`](https://www.zaber.com/protocol-manual#topic_setting_driver_enabled) setting.
    pub struct DriverEnabled: Setting<Type = bool, Name = "driver.enabled">, AxisScope;
    /// The type representing the [`driver.temperature`](https://www.zaber.com/protocol-manual#topic_setting_driver_temperature) setting.
    pub struct DriverTemperature: Setting<Type = f32, Name = "driver.temperature">, AxisScope;
    /// The type representing the [`encoder.count`](https://www.zaber.com/protocol-manual#topic_setting_encoder_count) setting.
    pub struct EncoderCount: Setting<Type = i64, Name = "encoder.count">, AxisScope;
    /// The type representing the [`encoder.count.cal`](https://www.zaber.com/protocol-manual#topic_setting_encoder_count_cal) setting.
    pub struct EncoderCountCal: Setting<Type = i64, Name = "encoder.count.cal">, AxisScope;
    /// The type representing the [`encoder.pos`](https://www.zaber.com/protocol-manual#topic_setting_encoder_pos) setting.
    pub struct EncoderPos: Setting<Type = i32, Name = "encoder.pos">, AxisScope;
    /// The type representing the [`encoder.pos.error`](https://www.zaber.com/protocol-manual#topic_setting_encoder_pos_error) setting.
    pub struct EncoderPosError: Setting<Type = i32, Name = "encoder.pos.error">, AxisScope;
    /// The type representing the [`knob.dir`](https://www.zaber.com/protocol-manual#topic_setting_knob_dir) setting.
    pub struct KnobDir: Setting<Type = bool, Name = "knob.dir">, AxisScope;
    /// The type representing the [`knob.distance`](https://www.zaber.com/protocol-manual#topic_setting_knob_distance) setting.
    pub struct KnobDistance: Setting<Type = u32, Name = "knob.distance">, AxisScope;
    /// The type representing the [`knob.enable`](https://www.zaber.com/protocol-manual#topic_setting_knob_enable) setting.
    pub struct KnobEnable: Setting<Type = bool, Name = "knob.enable">, AxisScope;
    /// The type representing the [`knob.maxspeed`](https://www.zaber.com/protocol-manual#topic_setting_knob_maxspeed) setting.
    pub struct KnobMaxspeed: Setting<Type = u64, Name = "knob.maxspeed">, AxisScope;
    /// The type representing the [`knob.mode`](https://www.zaber.com/protocol-manual#topic_setting_knob_mode) setting.
    pub struct KnobMode: Setting<Type = u8, Name = "knob.mode">, AxisScope;
    /// The type representing the [`knob.speedprofile`](https://www.zaber.com/protocol-manual#topic_setting_knob_speedprofile) setting.
    pub struct KnobSpeedprofile: Setting<Type = u8, Name = "knob.speedprofile">, AxisScope;
    /// The type representing the [`limit.approach.maxspeed`](https://www.zaber.com/protocol-manual#topic_setting_limit_approach_maxspeed) setting.
    pub struct LimitApproachMaxspeed: Setting<Type = u64, Name = "limit.approach.maxspeed">, AxisScope;
    /// The type representing the [`limit.cycle.dist`](https://www.zaber.com/protocol-manual#topic_setting_limit_cycle_dist) setting.
    pub struct LimitCycleDist: Setting<Type = u32, Name = "limit.cycle.dist">, AxisScope;
    /// The type representing the [`limit.detect.decelonly`](https://www.zaber.com/protocol-manual#topic_setting_limit_detect_decelonly) setting.
    pub struct LimitDetectDecelonly: Setting<Type = u32, Name = "limit.detect.decelonly">, AxisScope;
    /// The type representing the [`limit.detect.maxspeed`](https://www.zaber.com/protocol-manual#topic_setting_limit_detect_maxspeed) setting.
    pub struct LimitDetectMaxspeed: Setting<Type = u64, Name = "limit.detect.maxspeed">, AxisScope;
    /// The type representing the [`limit.home.action`](https://www.zaber.com/protocol-manual#topic_setting_limit_home_action) setting.
    pub struct LimitHomeAction: Setting<Type = u8, Name = "limit.home.action">, AxisScope;
    /// The type representing the [`limit.home.posupdate`](https://www.zaber.com/protocol-manual#topic_setting_limit_home_posupdate) setting.
    pub struct LimitHomePosupdate: Setting<Type = u8, Name = "limit.home.posupdate">, AxisScope;
    /// The type representing the [`limit.home.preset`](https://www.zaber.com/protocol-manual#topic_setting_limit_home_preset) setting.
    pub struct LimitHomePreset: Setting<Type = i32, Name = "limit.home.preset">, AxisScope;
    /// The type representing the [`limit.home.state`](https://www.zaber.com/protocol-manual#topic_setting_limit_home_state) setting.
    pub struct LimitHomeState: Setting<Type = bool, Name = "limit.home.state">, AxisScope;
    /// The type representing the [`limit.home.triggered`](https://www.zaber.com/protocol-manual#topic_setting_limit_home_triggered) setting.
    pub struct LimitHomeTriggered: Setting<Type = bool, Name = "limit.home.triggered">, AxisScope;
    /// The type representing the [`limit.max`](https://www.zaber.com/protocol-manual#topic_setting_limit_max) setting.
    pub struct LimitMax: Setting<Type = i32, Name = "limit.max">, AxisScope;
    /// The type representing the [`limit.min`](https://www.zaber.com/protocol-manual#topic_setting_limit_min) setting.
    pub struct LimitMin: Setting<Type = i32, Name = "limit.min">, AxisScope;
    /// The type representing the [`limit.start.pos`](https://www.zaber.com/protocol-manual#topic_setting_limit_start_pos) setting.
    pub struct LimitStartPos: Setting<Type = u8, Name = "limit.start.pos">, AxisScope;
    /// The type representing the [`maxspeed`](https://www.zaber.com/protocol-manual#topic_setting_maxspeed) setting.
    pub struct Maxspeed: Setting<Type = u64, Name = "maxspeed">, AxisScope;
    /// The type representing the [`motion.accelonly`](https://www.zaber.com/protocol-manual#topic_setting_motion_accelonly) setting.
    pub struct MotionAccelonly: Setting<Type = u32, Name = "motion.accelonly">, AxisScope;
    /// The type representing the [`motion.decelonly`](https://www.zaber.com/protocol-manual#topic_setting_motion_decelonly) setting.
    pub struct MotionDecelonly: Setting<Type = u32, Name = "motion.decelonly">, AxisScope;
    /// The type representing the [`motion.index.dist`](https://www.zaber.com/protocol-manual#topic_setting_motion_index_dist) setting.
    pub struct MotionIndexDist: Setting<Type = u32, Name = "motion.index.dist">, AxisScope;
    /// The type representing the [`motion.index.num`](https://www.zaber.com/protocol-manual#topic_setting_motion_index_num) setting.
    pub struct MotionIndexNum: Setting<Type = u32, Name = "motion.index.num">, AxisScope;
    /// The type representing the [`motor.current.max`](https://www.zaber.com/protocol-manual#topic_setting_motor_current_max) setting.
    pub struct MotorCurrentMax: Setting<Type = u32, Name = "motor.current.max">, AxisScope;
    /// The type representing the [`parking.state`](https://www.zaber.com/protocol-manual#topic_setting_parking_state) setting.
    pub struct ParkingState: Setting<Type = bool, Name = "parking.state">, AxisScope;
    /// The type representing the [`pos`](https://www.zaber.com/protocol-manual#topic_setting_pos) setting.
    pub struct Pos: Setting<Type = i32, Name = "pos">, AxisScope;
    /// The type representing the [`resolution`](https://www.zaber.com/protocol-manual#topic_setting_resolution) setting.
    pub struct Resolution: Setting<Type = u16, Name = "resolution">, AxisScope;
    /// The type representing the [`scope.delay`](https://www.zaber.com/protocol-manual#topic_setting_scope_delay) setting.
    pub struct ScopeDelay: Setting<Type = f32, Name = "scope.delay">, DeviceScope;
    /// The type representing the [`scope.timebase`](https://www.zaber.com/protocol-manual#topic_setting_scope_timebase) setting.
    pub struct ScopeTimebase: Setting<Type = f32, Name = "scope.timebase">, DeviceScope;
    /// The type representing the [`stream.numbufs`](https://www.zaber.com/protocol-manual#topic_setting_stream_numbufs) setting.
    pub struct StreamNumbufs: Setting<Type = u32, Name = "stream.numbufs">, DeviceScope;
    /// The type representing the [`stream.numstreams`](https://www.zaber.com/protocol-manual#topic_setting_stream_numstreams) setting.
    pub struct StreamNumstreams: Setting<Type = u32, Name = "stream.numstreams">, DeviceScope;
    /// The type representing the [`system.access`](https://www.zaber.com/protocol-manual#topic_setting_system_access) setting.
    pub struct SystemAccess: Setting<Type = u16, Name = "system.access">, DeviceScope;
    /// The type representing the [`system.axiscount`](https://www.zaber.com/protocol-manual#topic_setting_system_axiscount) setting.
    pub struct SystemAxiscount: Setting<Type = u32, Name = "system.axiscount">, DeviceScope;
    /// The type representing the [`system.led.enable`](https://www.zaber.com/protocol-manual#topic_setting_system_led_enable) setting.
    pub struct SystemLedEnable: Setting<Type = bool, Name = "system.led.enable">, DeviceScope;
    /// The type representing the [`system.serial`](https://www.zaber.com/protocol-manual#topic_setting_system_serial) setting.
    pub struct SystemSerial: Setting<Type = u32, Name = "system.serial">, DeviceScope;
    /// The type representing the [`system.voltage`](https://www.zaber.com/protocol-manual#topic_setting_system_voltage) setting.
    pub struct SystemVoltage: Setting<Type = f32, Name = "system.voltage">, DeviceScope;
    /// The type representing the [`version`](https://www.zaber.com/protocol-manual#topic_setting_version) setting.
    pub struct Version: Setting<Type = f32, Name = "version">, DeviceScope;
    /// The type representing the [`version.build`](https://www.zaber.com/protocol-manual#topic_setting_version_build) setting.
    pub struct VersionBuild: Setting<Type = u32, Name = "version.build">, DeviceScope;
}
define_any_setting! {
/// Any setting available in firmware version 7.05.
pub enum AnySetting {
    /// The [accel](https://www.zaber.com/protocol-manual#topic_setting_accel) setting.
    Accel,
    /// The [calibration.type](https://www.zaber.com/protocol-manual#topic_setting_calibration_type) setting.
    CalibrationType,
    /// The [cloop.enable](https://www.zaber.com/protocol-manual#topic_setting_cloop_enable) setting.
    CloopEnable,
    /// The [cloop.recovery.enable](https://www.zaber.com/protocol-manual#topic_setting_cloop_recovery_enable) setting.
    CloopRecoveryEnable,
    /// The [cloop.settle.period](https://www.zaber.com/protocol-manual#topic_setting_cloop_settle_period) setting.
    CloopSettlePeriod,
    /// The [cloop.settle.tolerance](https://www.zaber.com/protocol-manual#topic_setting_cloop_settle_tolerance) setting.
    CloopSettleTolerance,
    /// The [cloop.timeout](https://www.zaber.com/protocol-manual#topic_setting_cloop_timeout) setting.
    CloopTimeout,
    /// The [comm.address](https://www.zaber.com/protocol-manual#topic_setting_comm_address) setting.
    CommAddress,
    /// The [comm.alert](https://www.zaber.com/protocol-manual#topic_setting_comm_alert) setting.
    CommAlert,
    /// The [comm.checksum](https://www.zaber.com/protocol-manual#topic_setting_comm_checksum) setting.
    CommChecksum,
    /// The [comm.rs232.baud](https://www.zaber.com/protocol-manual#topic_setting_comm_rs232_baud) setting.
    CommRs232Baud,
    /// The [device.id](https://www.zaber.com/protocol-manual#topic_setting_device_id) setting.
    DeviceId,
    /// The [driver.current.hold](https://www.zaber.com/protocol-manual#topic_setting_driver_current_hold) setting.
    DriverCurrentHold,
    /// The [driver.current.max](https://www.zaber.com/protocol-manual#topic_setting_driver_current_max) setting.
    DriverCurrentMax,
    /// The [driver.current.run](https://www.zaber.com/protocol-manual#topic_setting_driver_current_run) setting.
    DriverCurrentRun,
    /// The [driver.current.servo](https://www.zaber.com/protocol-manual#topic_setting_driver_current_servo) setting.
    DriverCurrentServo,
    /// The [driver.enabled](https://www.zaber.com/protocol-manual#topic_setting_driver_enabled) setting.
    DriverEnabled,
    /// The [driver.temperature](https://www.zaber.com/protocol-manual#topic_setting_driver_temperature) setting.
    DriverTemperature,
    /// The [encoder.count](https://www.zaber.com/protocol-manual#topic_setting_encoder_count) setting.
    EncoderCount,
    /// The [encoder.count.cal](https://www.zaber.com/protocol-manual#topic_setting_encoder_count_cal) setting.
    EncoderCountCal,
    /// The [encoder.pos](https://www.zaber.com/protocol-manual#topic_setting_encoder_pos) setting.
    EncoderPos,
    /// The [encoder.pos.error](https://www.zaber.com/protocol-manual#topic_setting_encoder_pos_error) setting.
    EncoderPosError,
    /// The [knob.dir](https://www.zaber.com/protocol-manual#topic_setting_knob_dir) setting.
    KnobDir,
    /// The [knob.distance](https://www.zaber.com/protocol-manual#topic_setting_knob_distance) setting.
    KnobDistance,
    /// The [knob.enable](https://www.zaber.com/protocol-manual#topic_setting_knob_enable) setting.
    KnobEnable,
    /// The [knob.maxspeed](https://www.zaber.com/protocol-manual#topic_setting_knob_maxspeed) setting.
    KnobMaxspeed,
    /// The [knob.mode](https://www.zaber.com/protocol-manual#topic_setting_knob_mode) setting.
    KnobMode,
    /// The [knob.speedprofile](https://www.zaber.com/protocol-manual#topic_setting_knob_speedprofile) setting.
    KnobSpeedprofile,
    /// The [limit.approach.maxspeed](https://www.zaber.com/protocol-manual#topic_setting_limit_approach_maxspeed) setting.
    LimitApproachMaxspeed,
    /// The [limit.cycle.dist](https://www.zaber.com/protocol-manual#topic_setting_limit_cycle_dist) setting.
    LimitCycleDist,
    /// The [limit.detect.decelonly](https://www.zaber.com/protocol-manual#topic_setting_limit_detect_decelonly) setting.
    LimitDetectDecelonly,
    /// The [limit.detect.maxspeed](https://www.zaber.com/protocol-manual#topic_setting_limit_detect_maxspeed) setting.
    LimitDetectMaxspeed,
    /// The [limit.home.action](https://www.zaber.com/protocol-manual#topic_setting_limit_home_action) setting.
    LimitHomeAction,
    /// The [limit.home.posupdate](https://www.zaber.com/protocol-manual#topic_setting_limit_home_posupdate) setting.
    LimitHomePosupdate,
    /// The [limit.home.preset](https://www.zaber.com/protocol-manual#topic_setting_limit_home_preset) setting.
    LimitHomePreset,
    /// The [limit.home.state](https://www.zaber.com/protocol-manual#topic_setting_limit_home_state) setting.
    LimitHomeState,
    /// The [limit.home.triggered](https://www.zaber.com/protocol-manual#topic_setting_limit_home_triggered) setting.
    LimitHomeTriggered,
    /// The [limit.max](https://www.zaber.com/protocol-manual#topic_setting_limit_max) setting.
    LimitMax,
    /// The [limit.min](https://www.zaber.com/protocol-manual#topic_setting_limit_min) setting.
    LimitMin,
    /// The [limit.start.pos](https://www.zaber.com/protocol-manual#topic_setting_limit_start_pos) setting.
    LimitStartPos,
    /// The [maxspeed](https://www.zaber.com/protocol-manual#topic_setting_maxspeed) setting.
    Maxspeed,
    /// The [motion.accelonly](https://www.zaber.com/protocol-manual#topic_setting_motion_accelonly) setting.
    MotionAccelonly,
    /// The [motion.decelonly](https://www.zaber.com/protocol-manual#topic_setting_motion_decelonly) setting.
    MotionDecelonly,
    /// The [motion.index.dist](https://www.zaber.com/protocol-manual#topic_setting_motion_index_dist) setting.
    MotionIndexDist,
    /// The [motion.index.num](https://www.zaber.com/protocol-manual#topic_setting_motion_index_num) setting.
    MotionIndexNum,
    /// The [motor.current.max](https://www.zaber.com/protocol-manual#topic_setting_motor_current_max) setting.
    MotorCurrentMax,
    /// The [parking.state](https://www.zaber.com/protocol-manual#topic_setting_parking_state) setting.
    ParkingState,
    /// The [pos](https://www.zaber.com/protocol-manual#topic_setting_pos) setting.
    Pos,
    /// The [resolution](https://www.zaber.com/protocol-manual#topic_setting_resolution) setting.
    Resolution,
    /// The [scope.delay](https://www.zaber.com/protocol-manual#topic_setting_scope_delay) setting.
    ScopeDelay,
    /// The [scope.timebase](https://www.zaber.com/protocol-manual#topic_setting_scope_timebase) setting.
    ScopeTimebase,
    /// The [stream.numbufs](https://www.zaber.com/protocol-manual#topic_setting_stream_numbufs) setting.
    StreamNumbufs,
    /// The [stream.numstreams](https://www.zaber.com/protocol-manual#topic_setting_stream_numstreams) setting.
    StreamNumstreams,
    /// The [system.access](https://www.zaber.com/protocol-manual#topic_setting_system_access) setting.
    SystemAccess,
    /// The [system.axiscount](https://www.zaber.com/protocol-manual#topic_setting_system_axiscount) setting.
    SystemAxiscount,
    /// The [system.led.enable](https://www.zaber.com/protocol-manual#topic_setting_system_led_enable) setting.
    SystemLedEnable,
    /// The [system.serial](https://www.zaber.com/protocol-manual#topic_setting_system_serial) setting.
    SystemSerial,
    /// The [system.voltage](https://www.zaber.com/protocol-manual#topic_setting_system_voltage) setting.
    SystemVoltage,
    /// The [version](https://www.zaber.com/protocol-manual#topic_setting_version) setting.
    Version,
    /// The [version.build](https://www.zaber.com/protocol-manual#topic_setting_version_build) setting.
    VersionBuild,
}
}
