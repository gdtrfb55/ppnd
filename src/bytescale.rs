//  This file is part of the ppnd distribution, which can be found at 
//  <https://github.com/gdtrfb55/ppnd>.
//
//  Copyright (C) 2021 Jack Browning.
//
//  This program is FREE SOFTWARE: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

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

const fn max_precision_or(sig_digits: usize) -> usize {
    use crate::options::MAX_PRECISION;
    
    if sig_digits > MAX_PRECISION { MAX_PRECISION } else { sig_digits }
}

fn scale_format(count: u64, scale: &Scale) -> (u64, usize, String) {
    const B_MAX_PREC: usize = 0;
    const K_MAX_PREC: usize = max_precision_or(3);
    const M_MAX_PREC: usize = max_precision_or(6);
    const G_MAX_PREC: usize = max_precision_or(9);
    const T_MAX_PREC: usize = max_precision_or(12);
    const P_MAX_PREC: usize = max_precision_or(15);

    const B_DIV: u64 = 1;
    const B_SUFF: &str = "B";

    const KB_DIV: u64 = 1000;
    const KB_SUFF: &str = "kB";

    const MB_DIV: u64 = KB_DIV.pow(2);
    const MB_SUFF: &str = "MB";

    const GB_DIV: u64 = KB_DIV.pow(3);
    const GB_SUFF: &str = "GB";

    const TB_DIV: u64 = KB_DIV.pow(4);
    const TB_SUFF: &str = "TB";

    const PB_DIV: u64 = KB_DIV.pow(5);
    const PB_SUFF: &str = "PB";

    const KIB_DIV: u64 = 1024;
    const KIB_SUFF: &str = "KiB";

    const MIB_DIV: u64 = KIB_DIV.pow(2);
    const MIB_SUFF: &str = "MiB";

    const GIB_DIV: u64 = KIB_DIV.pow(3);
    const GIB_SUFF: &str = "GiB";

    const TIB_DIV: u64 = KIB_DIV.pow(4);
    const TIB_SUFF: &str = "TiB";

    const PIB_DIV: u64 = KIB_DIV.pow(5);
    const PIB_SUFF: &str = "PiB";

    match scale {
        Scale::Dyn10 => if count < KB_DIV {
            scale_format(count, &Scale::Raw)
        } else if count < MB_DIV {
            scale_format(count, &Scale::Kilo)
        } else if count < GB_DIV {
            scale_format(count, &Scale::Mega)
        } else if count < TB_DIV {
            scale_format(count, &Scale::Giga)
        } else if count < PB_DIV {
            scale_format(count, &Scale::Tera)
        } else {
            scale_format(count, &Scale::Peta)
        },
        Scale::Dyn2 => if count < KIB_DIV {
            scale_format(count, &Scale::Raw)
        } else if count < MIB_DIV {
            scale_format(count, &Scale::Kibi)
        } else if count < GIB_DIV {
            scale_format(count, &Scale::Mebi)
        } else if count < TIB_DIV {
            scale_format(count, &Scale::Gibi)
        } else if count < PIB_DIV {
            scale_format(count, &Scale::Tebi)
        } else {
            scale_format(count, &Scale::Pebi)
        },
        Scale::Raw => (B_DIV, B_MAX_PREC, B_SUFF.to_string()),
        Scale::Kilo => (KB_DIV, K_MAX_PREC, KB_SUFF.to_string()),
        Scale::Mega => (MB_DIV, M_MAX_PREC, MB_SUFF.to_string()),
        Scale::Giga => (GB_DIV, G_MAX_PREC, GB_SUFF.to_string()),
        Scale::Tera => (TB_DIV, T_MAX_PREC, TB_SUFF.to_string()),
        Scale::Peta => (PB_DIV, P_MAX_PREC, PB_SUFF.to_string()),
        Scale::Kibi => (KIB_DIV, K_MAX_PREC, KIB_SUFF.to_string()),
        Scale::Mebi => (MIB_DIV, M_MAX_PREC, MIB_SUFF.to_string()),
        Scale::Gibi => (GIB_DIV, G_MAX_PREC, GIB_SUFF.to_string()),
        Scale::Tebi => (TIB_DIV, T_MAX_PREC, TIB_SUFF.to_string()),
        Scale::Pebi => (PIB_DIV, P_MAX_PREC, PIB_SUFF.to_string())
    }
}

pub fn valid_scale(s: String) -> Result<Scale, String> {
    match s.as_str() {
        "dyn10" => Ok(Scale::Dyn10),
        "dyn2" => Ok(Scale::Dyn2),
        "raw" => Ok(Scale::Raw),
        "kb" => Ok(Scale::Kilo),
        "mb" => Ok(Scale::Mega),
        "gb" => Ok(Scale::Giga),
        "tb" => Ok(Scale::Tera),
        "pb" => Ok(Scale::Peta),
        "kib" => Ok(Scale::Kibi),
        "mib" => Ok(Scale::Mebi),
        "gib" => Ok(Scale::Gibi),
        "tib" => Ok(Scale::Tebi),
        "pib" => Ok(Scale::Pebi),
        _ => Err(format!("'{}' is not a valid scale value", s))
    }
}

pub fn scale(count: u64, scale: &Scale, requested_prec: usize) -> String {
    let (div, usable_prec, suff) = scale_format(count, scale);
    let scaled = count as f64 / div as f64;
    format!("{0:.1$} {2}", scaled, std::cmp::min(usable_prec, requested_prec), suff)
}
