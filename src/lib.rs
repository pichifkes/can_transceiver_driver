// #[derive(...)] automatically writes code so we can copy, print (in tests),
// and compare CanFrames using `==`.
#![no_std]

fn float_to_clamped_uint(val: f32, min_val: f32, max_val: f32, num_bits: u16) -> u64 {
    let clamped_val = val.max(min_val).min(max_val);
    let normalized_val = (clamped_val - min_val) / (max_val - min_val);
    let scale = ((1u32 << num_bits) - 1) as f32;
    (normalized_val * scale as f32) as u64
}

pub fn pack_mit(p_des: f32, v_des: f32, kp: f32, kd: f32, t: f32) -> [u8; 8] {
    const P_MIN: f32 = -12.5;
    const P_MAX: f32 = 12.5;
    const V_MIN: f32 = -65.0;
    const V_MAX: f32 = 65.0;
    const KP_MIN: f32 = 0.0;
    const KP_MAX: f32 = 500.0;
    const KD_MIN: f32 = 0.0;
    const KD_MAX: f32 = 5.0;
    const T_MIN: f32 = -18.0;
    const T_MAX: f32 = 18.0;

    let some_numba: u64 = ((float_to_clamped_uint(p_des, P_MIN, P_MAX, 16) as u64) << 48)
        | ((float_to_clamped_uint(v_des, V_MIN, V_MAX, 12) as u64) << 36)
        | ((float_to_clamped_uint(kp, KP_MIN, KP_MAX, 12) as u64) << 24)
        | ((float_to_clamped_uint(kd, KD_MIN, KD_MAX, 12) as u64) << 12)
        | ((float_to_clamped_uint(t, T_MIN, T_MAX, 12) as u64) << 0);

    return some_numba.to_be_bytes();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanFrame {
    pub id: u16,
    pub dlc: u8,
    pub data: [u8; 8],
}
impl CanFrame {
    pub fn new(id: u16, payload: &[u8]) -> Option<Self> {
        if id > 0b11111111111 {
            return None;
        }

        if payload.len() > 8 {
            return None;
        }

        let mut data = [0u8; 8];
        data[..payload.len()].copy_from_slice(payload);

        return Some(CanFrame {
            id,
            dlc: payload.len() as u8,
            data: data,
        });
    }
}

pub fn motor_command(
    motor_id: u16,
    p_des: f32,
    v_des: f32,
    kp: f32,
    kd: f32,
    t: f32,
) -> Option<CanFrame> {
    let pizza_pasta = pack_mit(p_des, v_des, kp, kd, t);
    return CanFrame::new(motor_id, &pizza_pasta);
}

pub struct Crc15 {
    crc: u16,
}

impl Crc15 {
    pub fn new() -> Self {
        return Crc15 { crc: 0 };
    }

    /// Feed a single bit (true = 1, false = 0) into the CRC calculator
    pub fn update_bit(&mut self, bit: bool) {
        let bit_u16 = if bit { 1 } else { 0 }; //coverting it to a number
        let crc_next = bit_u16 ^ ((self.crc >> 14) & 1); //getting the first bit

        self.crc = (self.crc << 1) & 0b1111111111111111;

        if crc_next != 0 {
            self.crc ^= 0b100010110011001; //standard generator polynomial used for CRC-15-CAN
        }
    }

    /// Read out the final 15-bit CRC value
    pub fn value(&self) -> u16 {
        self.crc
    }
}
