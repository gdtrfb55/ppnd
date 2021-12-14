/*
    This file is part of the ppnd distribution, which can be found at
    <https://github.com/gdtrfb55/ppnd>.

    Copyright (C) 2021 Jack Browning.

    This program is FREE SOFTWARE: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

const fn max_precision_or(sig_digits: usize) -> usize {
    use crate::options::Precision;

    if sig_digits > Precision::MAX { Precision::MAX } else { sig_digits }
}

const B_MAX_PREC: usize = 0;
const K_MAX_PREC: usize = max_precision_or(3);
const M_MAX_PREC: usize = max_precision_or(6);
const G_MAX_PREC: usize = max_precision_or(9);
const T_MAX_PREC: usize = max_precision_or(12);
const P_MAX_PREC: usize = max_precision_or(15);

const B_DIV: u64 = 1;
const B_FORM: (u64, usize, &str) = (B_DIV, B_MAX_PREC, "B");

const KB_DIV: u64 = 1000;
const KB_FORM: (u64, usize, &str) = (KB_DIV, K_MAX_PREC, "kB");

const MB_DIV: u64 = KB_DIV.pow(2);
const MB_FORM: (u64, usize, &str) = (MB_DIV, M_MAX_PREC, "MB");

const GB_DIV: u64 = KB_DIV.pow(3);
const GB_FORM: (u64, usize, &str) = (GB_DIV, G_MAX_PREC, "GB");

const TB_DIV: u64 = KB_DIV.pow(4);
const TB_FORM: (u64, usize, &str) = (TB_DIV, T_MAX_PREC, "TB");

const PB_DIV: u64 = KB_DIV.pow(5);
const PB_FORM: (u64, usize, &str) = (PB_DIV, P_MAX_PREC, "PB");

const KIB_DIV: u64 = 1024;
const KIB_FORM: (u64, usize, &str) = (KIB_DIV, K_MAX_PREC, "KiB");

const MIB_DIV: u64 = KIB_DIV.pow(2);
const MIB_FORM: (u64, usize, &str) = (MIB_DIV, M_MAX_PREC, "MiB");

const GIB_DIV: u64 = KIB_DIV.pow(3);
const GIB_FORM: (u64, usize, &str) = (GIB_DIV, G_MAX_PREC, "GiB");

const TIB_DIV: u64 = KIB_DIV.pow(4);
const TIB_FORM: (u64, usize, &str) = (TIB_DIV, T_MAX_PREC, "TiB");

const PIB_DIV: u64 = KIB_DIV.pow(5);
const PIB_FORM: (u64, usize, &str) = (PIB_DIV, P_MAX_PREC, "PiB");

pub enum Scale {
    Dyn10,
    Dyn2,
    Raw,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Kibi,
    Mebi,
    Gibi,
    Tebi,
    Pebi
}

impl Scale {
    pub const DEFAULT: Scale = Scale::Dyn10;

    pub fn from_string(s: String) -> Result<Scale, String> {
        match s.as_str() {
            "dyn" | "dyn10" => Ok(Self::Dyn10),
            "dyn2" => Ok(Self::Dyn2),
            "raw" => Ok(Self::Raw),
            "kb" => Ok(Self::Kilo),
            "mb" => Ok(Self::Mega),
            "gb" => Ok(Self::Giga),
            "tb" => Ok(Self::Tera),
            "pb" => Ok(Self::Peta),
            "kib" => Ok(Self::Kibi),
            "mib" => Ok(Self::Mebi),
            "gib" => Ok(Self::Gibi),
            "tib" => Ok(Self::Tebi),
            "pib" => Ok(Self::Pebi),
            _ => Err(format!("'{}' is not a valid scale value", s))
        }
    }

    fn format(self: &Self, count: u64) -> (u64, usize, &'static str) {
        match self {
            Self::Dyn10 => {
                let s = match count {
                    c if c < KB_DIV => Self::Raw,
                    c if c < MB_DIV => Self::Kilo,
                    c if c < GB_DIV => Self::Mega,
                    c if c < TB_DIV => Self::Giga,
                    c if c < PB_DIV => Self::Tera,
                    _ => Self::Peta
                };
                s.format(count)
            },
            Self::Dyn2 => {
                let s = match count {
                    c if c < KIB_DIV => Self::Raw,
                    c if c < MIB_DIV => Self::Kibi,
                    c if c < GIB_DIV => Self::Mebi,
                    c if c < TIB_DIV => Self::Gibi,
                    c if c < PIB_DIV => Self::Tebi,
                    _ => Self::Pebi
                };
                s.format(count)
            },
            Self::Raw => B_FORM,
            Self::Kilo => KB_FORM,
            Self::Mega => MB_FORM,
            Self::Giga => GB_FORM,
            Self::Tera => TB_FORM,
            Self::Peta => PB_FORM,
            Self::Kibi => KIB_FORM,
            Self::Mebi => MIB_FORM,
            Self::Gibi => GIB_FORM,
            Self::Tebi => TIB_FORM,
            Self::Pebi => PIB_FORM
        }
    }

    pub fn scaled_bytes(self: &Self, count: u64, requested_prec: usize) -> String {
        let (divisor, usable_prec, suffix) = self.format(count);
        let scaled = count as f64 / divisor as f64;
        format!("{0:.1$} {2}", scaled, std::cmp::min(usable_prec, requested_prec), suffix)
    }
}
