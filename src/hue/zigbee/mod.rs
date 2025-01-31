use std::io::{Read, Write};

use bitflags::bitflags;
use byteorder::{LittleEndian as LE, ReadBytesExt, WriteBytesExt};
use packed_struct::derive::{PackedStruct, PrimitiveEnum_u8};
use packed_struct::{PackedStruct, PackedStructSlice, PrimitiveEnum};

use crate::error::{ApiError, ApiResult};
use crate::model::flags::TakeFlag;
use crate::model::types::XY;

#[derive(PrimitiveEnum_u8, Debug, Copy, Clone)]
pub enum EffectType {
    NoEffect = 0x00,
    Candle = 0x01,
    Fireplace = 0x02,
    Prism = 0x03,
    Sunrise = 0x09,
    Sparkle = 0x0a,
    Opal = 0x0b,
    Glisten = 0x0c,
    Underwater = 0x0e,
    Cosmos = 0x0f,
    Sunbeam = 0x10,
    Enchant = 0x11,
}

#[derive(PrimitiveEnum_u8, Debug, Copy, Clone)]
pub enum GradientStyle {
    Linear = 0x00,
    Scattered = 0x02,
    Mirrored = 0x04,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: u16 {
        const ON_OFF          = 1 <<  0;
        const BRIGHTNESS      = 1 <<  1;
        const COLOR_MIREK     = 1 <<  2;
        const COLOR_XY        = 1 <<  3;
        const UNKNOWN_0       = 1 <<  4;
        const EFFECT_TYPE     = 1 <<  5;
        const GRADIENT_PARAMS = 1 <<  6;
        const EFFECT_SPEED    = 1 <<  7;
        const GRADIENT_COLORS = 1 <<  8;
        const UNUSED_9        = 1 <<  9;
        const UNUSED_A        = 1 << 10;
        const UNUSED_B        = 1 << 11;
        const UNUSED_C        = 1 << 12;
        const UNUSED_D        = 1 << 13;
        const UNUSED_E        = 1 << 14;
        const UNUSED_F        = 1 << 15;
    }
}

#[derive(Default, PackedStruct)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
pub struct GradientUpdateHeader {
    /// First 4 bits of first byte: number of gradient light points
    #[packed_field(bits = "0..4")]
    pub nlights: u8,

    /// Last 4 bits of first byte: unknown
    #[packed_field(bits = "4..8")]
    pub resv0: u8,

    /// Second byte: unknown
    pub resv1: u8,

    /// Third and fourth byte: unknown
    pub resv2: u16,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(endian = "lsb")]
struct PackedXY12 {
    #[packed_field(size_bits = "12")]
    x: u16,
    #[packed_field(size_bits = "12")]
    y: u16,
}

pub struct GradientColors {
    pub header: GradientUpdateHeader,
    pub points: Vec<XY>,
}

#[derive(Debug, PackedStruct)]
#[packed_struct(endian = "lsb")]
pub struct GradientParams {
    pub scale: u8,
    pub offset: u8,
}

#[derive(Default)]
pub struct HueZigbeeUpdate {
    pub onoff: Option<u8>,
    pub brightness: Option<u8>,
    pub color_mirek: Option<u16>,
    pub color_xy: Option<XY>,
    pub unk0: Option<u16>,
    pub gradient_colors: Option<GradientColors>,
    pub gradient_params: Option<GradientParams>,
    pub effect_type: Option<EffectType>,
    pub effect_speed: Option<u8>,
}

impl HueZigbeeUpdate {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_on_off(mut self, on_off: bool) -> Self {
        self.onoff = Some(if on_off { 1 } else { 0 });
        self
    }

    #[must_use]
    pub const fn with_brightness(mut self, brightness: u8) -> Self {
        self.brightness = Some(brightness);
        self
    }

    #[must_use]
    pub const fn with_color_mirek(mut self, mirek: u16) -> Self {
        self.color_mirek = Some(mirek);
        self
    }

    #[must_use]
    pub const fn with_color_xy(mut self, xy: XY) -> Self {
        self.color_xy = Some(xy);
        self
    }

    #[must_use]
    pub const fn with_unknown0(mut self, unk0: u16) -> Self {
        self.unk0 = Some(unk0);
        self
    }

    #[must_use]
    pub fn with_gradient_colors(mut self, colors: GradientColors) -> Self {
        self.gradient_colors = Some(colors);
        self
    }

    #[must_use]
    pub const fn with_gradient_transform(mut self, transform: GradientParams) -> Self {
        self.gradient_params = Some(transform);
        self
    }

    #[must_use]
    pub const fn with_effect_type(mut self, effect_type: EffectType) -> Self {
        self.effect_type = Some(effect_type);
        self
    }

    #[must_use]
    pub const fn with_effect_speed(mut self, effect_speed: u8) -> Self {
        self.effect_speed = Some(effect_speed);
        self
    }
}

#[allow(clippy::cast_possible_truncation)]
impl HueZigbeeUpdate {
    pub fn from_reader(rdr: &mut impl Read) -> ApiResult<Self> {
        let mut hz = Self::default();

        let mut flags = Flags::from_bits(rdr.read_u16::<LE>()?).unwrap();

        if flags.take(Flags::ON_OFF) {
            hz.onoff = Some(rdr.read_u8()?);
        }

        if flags.take(Flags::BRIGHTNESS) {
            hz.brightness = Some(rdr.read_u8()?);
        }

        if flags.take(Flags::COLOR_MIREK) {
            hz.color_mirek = Some(rdr.read_u16::<LE>()?);
        }

        if flags.take(Flags::COLOR_XY) {
            hz.color_xy = Some(XY::new(
                f64::from(rdr.read_u16::<LE>()?) / f64::from(0xFFFF),
                f64::from(rdr.read_u16::<LE>()?) / f64::from(0xFFFF),
            ));
        }

        if flags.take(Flags::UNKNOWN_0) {
            hz.unk0 = Some(rdr.read_u16::<LE>()?);
        }

        if flags.take(Flags::EFFECT_TYPE) {
            let data = rdr.read_u8()?;
            hz.effect_type =
                Some(EffectType::from_primitive(data).ok_or(ApiError::HueZigbeeDecodeError)?);
        }

        if flags.take(Flags::GRADIENT_COLORS) {
            let len = rdr.read_u8()?;
            let mut data = vec![0; 4];
            rdr.read_exact(&mut data)?;
            let header = GradientUpdateHeader::unpack_from_slice(&data)?;
            debug_assert!(len == header.nlights * 3 + 4);

            let mut points = vec![];
            for _ in 0..header.nlights {
                let mut point = vec![0; 3];
                rdr.read_exact(&mut point)?;
                let point = PackedXY12::unpack_from_slice(&point)?;
                points.push(XY {
                    x: f64::from(point.x) / f64::from(0xFFF),
                    y: f64::from(point.y) / f64::from(0xFFF),
                });
            }
            hz.gradient_colors = Some(GradientColors { header, points });
        }

        if flags.take(Flags::EFFECT_SPEED) {
            hz.effect_speed = Some(rdr.read_u8()?);
        }

        if flags.take(Flags::GRADIENT_PARAMS) {
            hz.gradient_params = Some(GradientParams {
                scale: rdr.read_u8()?,
                offset: rdr.read_u8()?,
            });
        }

        if flags.is_empty() {
            Ok(hz)
        } else {
            Err(ApiError::HueZigbeeUnknownFlags(flags.bits()))
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl HueZigbeeUpdate {
    pub fn serialize(&self, wtr: &mut impl Write) -> ApiResult<()> {
        #[allow(clippy::ref_option)]
        fn opt_to_flag<T>(flags: &mut Flags, opt: &Option<T>, flag: Flags) {
            if opt.is_some() {
                flags.insert(flag);
            }
        }

        let mut flags = Flags::empty();
        opt_to_flag(&mut flags, &self.onoff, Flags::ON_OFF);
        opt_to_flag(&mut flags, &self.brightness, Flags::BRIGHTNESS);
        opt_to_flag(&mut flags, &self.color_mirek, Flags::COLOR_MIREK);
        opt_to_flag(&mut flags, &self.color_xy, Flags::COLOR_XY);
        opt_to_flag(&mut flags, &self.unk0, Flags::UNKNOWN_0);
        opt_to_flag(&mut flags, &self.effect_type, Flags::EFFECT_TYPE);
        opt_to_flag(&mut flags, &self.effect_speed, Flags::EFFECT_SPEED);
        opt_to_flag(&mut flags, &self.gradient_colors, Flags::GRADIENT_COLORS);
        opt_to_flag(&mut flags, &self.gradient_params, Flags::GRADIENT_PARAMS);

        wtr.write_u16::<LE>(flags.bits())?;

        if let Some(onoff) = self.onoff {
            wtr.write_u8(onoff)?;
        }

        if let Some(bright) = self.brightness {
            wtr.write_u8(bright)?;
        }

        if let Some(mirek) = self.color_mirek {
            wtr.write_u16::<LE>(mirek)?;
        }

        if let Some(xy) = self.color_xy {
            wtr.write_u16::<LE>((xy.x * f64::from(0xFFFF)) as u16)?;
            wtr.write_u16::<LE>((xy.y * f64::from(0xFFFF)) as u16)?;
        }

        if let Some(unk0) = self.unk0 {
            wtr.write_u16::<LE>(unk0)?;
        }

        if let Some(etype) = self.effect_type {
            wtr.write_u8(etype.to_primitive())?;
        }

        if let Some(grad_color) = &self.gradient_colors {
            let len = u8::try_from(4 + 3 * grad_color.points.len())?;
            wtr.write_u8(len)?;
            wtr.write_all(&grad_color.header.pack()?)?;
            for point in &grad_color.points {
                let packed = PackedXY12 {
                    x: (point.x * f64::from(0xFFF)) as u16,
                    y: (point.y * f64::from(0xFFF)) as u16,
                };
                wtr.write_all(&packed.pack()?)?;
            }
        }

        if let Some(effect_speed) = self.effect_speed {
            wtr.write_u8(effect_speed)?;
        }

        if let Some(params) = &self.gradient_params {
            wtr.write_u8(params.scale)?;
            wtr.write_u8(params.offset)?;
        }

        Ok(())
    }
}
