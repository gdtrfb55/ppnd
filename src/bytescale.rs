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

fn scale_format(bytes: u64, scale: &Scale) -> (u64, usize, String) {
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
    Scale::Dyn10 => if bytes < KB_DIV {
      scale_format(bytes, &Scale::Raw)
    } else if bytes < MB_DIV {
      scale_format(bytes, &Scale::Kilo)
    } else if bytes < GB_DIV {
      scale_format(bytes, &Scale::Mega)
    } else if bytes < TB_DIV {
      scale_format(bytes, &Scale::Giga)
    } else if bytes < PB_DIV {
      scale_format(bytes, &Scale::Tera)
    } else {
      scale_format(bytes, &Scale::Peta)
    },
    Scale::Dyn2 => if bytes < KIB_DIV {
      scale_format(bytes, &Scale::Raw)
    } else if bytes < MIB_DIV {
      scale_format(bytes, &Scale::Kibi)
    } else if bytes < GIB_DIV {
      scale_format(bytes, &Scale::Mebi)
    } else if bytes < TIB_DIV {
      scale_format(bytes, &Scale::Gibi)
    } else if bytes < PIB_DIV {
      scale_format(bytes, &Scale::Tebi)
    } else {
      scale_format(bytes, &Scale::Pebi)
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

pub fn scale(bytes: u64, scale: &Scale, requested_prec: usize) -> String {
  let (div, usable_prec, suff) = scale_format(bytes, scale);
  let scaled = bytes as f64 / div as f64;
  format!("{0:.1$} {2}", scaled, std::cmp::min(usable_prec, requested_prec), suff)
}