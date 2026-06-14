#![allow(unused_variables, dead_code)]


#[derive(Default)]
struct ChannelOne {
    nr10_sweep: SweepReg,
    nr11_ln_timer_duty_cycle: LnTimerDutyCycleReg,
    nr12_volume_envelope: VolumeEnvReg,
    nr13_period_high_ctrl: PeriodHighCtrlReg,
    nr14_period_low: PeriodLowReg,
}

#[derive(Default)]
struct ChannelTwo {
    nr21_ln_timer_duty_cycle: LnTimerDutyCycleReg,
    nr22_volume_envelope: VolumeEnvReg,
    nr23_period_high_ctrl: PeriodHighCtrlReg,
    nr24_period_low: PeriodLowReg,
}

#[derive(Default)]
struct ChannelThree {
    nr30_dac_enable: WaveDacEnableReg,
    nr31_ln_timer: WaveLengthTimerReg,
    nr32_output_level: OutputLevelReg,
    nr33_period_low: PeriodLowReg,
    nr34_period_high_crtl: PeriodHighCtrlReg,
}

#[derive(Default)]
struct ChannelFour {
    nr41_length_timer: NoiseLengthTimer,
    nr42_volume_envelope: VolumeEnvReg,
    nr43_freq_and_randomness: FreqRandomnessReg,
    nr44_control: ChannelFourCtrlReg,
}

#[derive(Default)]
pub struct Apu {
    audio_master_control: AudioMasterControlReg,
    sound_panning: SoundPanningReg,
    master_vol_and_vin_panning: MasterVolVinPanningReg,

    channel_one: ChannelOne,
    channel_two: ChannelTwo,
    channel_three: ChannelThree,
    channel_four: ChannelFour,
}

trait Channel {}

macro_rules! define_register {
    ($name:ident) => {
        #[derive(Default, Debug, Copy, Clone)]
        struct $name { byte: u8, }
    };
}

macro_rules! read_write_register {
    ($name:ident) => {
        define_register!($name);
        impl Register for $name {
            fn read(&self) -> u8 { self.byte }
            fn write(&mut self, value: u8) { self.byte = value}
        }
    };
}

macro_rules! write_only_register {
    ($name:ident) => {
        define_register!($name);
        impl Register for $name {
            fn read(&self) -> u8 { 0xFF }
            fn write(&mut self, value: u8) { self.byte = value}
        }
    };
}

read_write_register!(AudioMasterControlReg);
read_write_register!(SoundPanningReg);
read_write_register!(MasterVolVinPanningReg);
read_write_register!(SweepReg);

define_register!(LnTimerDutyCycleReg);
impl Register for LnTimerDutyCycleReg {
    fn read(&self) -> u8 { (self.byte & 0b1100_0000) | 0b0011_1111 }
    fn write(&mut self, value: u8) { self.byte = value;}
}

read_write_register!(VolumeEnvReg);
write_only_register!(PeriodHighCtrlReg);
write_only_register!(PeriodLowReg);
read_write_register!(WaveDacEnableReg);
write_only_register!(WaveLengthTimerReg);

read_write_register!(OutputLevelReg);
read_write_register!(NoiseLengthTimer);
read_write_register!(FreqRandomnessReg);
define_register!(ChannelFourCtrlReg);
impl Register for ChannelFourCtrlReg {
    fn read(&self) -> u8 { (self.byte & 0b0110_0000) | 0b1001_1111 }
    fn write(&mut self, value: u8) { self.byte = value }
}

trait Register {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
}

impl Apu {
    pub fn read(&self, addr: u16) -> u8 { 0xFF }
    pub fn write(&mut self, addr:u16 , value :u8) { }
}
