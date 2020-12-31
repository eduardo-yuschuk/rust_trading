#[derive(Clone, Debug)]
pub enum RetCode {
   Success,
   BadParam,
   OutOfRangeStartIndex,
   OutOfRangeEndIndex,
   AllocErr,
   InternalError,
}

//pub struct MInteger {
//   pub value: i32,
//}

pub fn sma_lookback(mut opt_in_time_period: i32) -> i32 {
   if opt_in_time_period == std::i32::MIN {
      opt_in_time_period = 30;
   } else if opt_in_time_period < 2i32 || opt_in_time_period > 100000i32 {
      return -1;
   }
   return opt_in_time_period - 1;
}

pub fn sma(
   start_idx: i32,
   end_idx: i32,
   in_real: &Vec<f64>,
   mut opt_in_time_period: i32,
   out_beg_idx: &mut i32,
   out_nb_element: &mut i32,
   out_real: &mut Vec<f64>,
) -> RetCode {
   if start_idx < 0 {
      return RetCode::OutOfRangeStartIndex;
   }
   if end_idx < 0 || end_idx < start_idx {
      return RetCode::OutOfRangeEndIndex;
   }
   if opt_in_time_period == std::i32::MIN {
      opt_in_time_period = 30;
   } else if opt_in_time_period < 2 || opt_in_time_period > 100000 {
      return RetCode::BadParam;
   }
   return ta_int_sma(
      start_idx,
      end_idx,
      in_real,
      opt_in_time_period,
      out_beg_idx,
      out_nb_element,
      out_real,
   );
}

fn ta_int_sma(
   mut start_idx: i32,
   end_idx: i32,
   in_real: &Vec<f64>,
   opt_in_time_period: i32,
   out_beg_idx: &mut i32,
   out_nb_element: &mut i32,
   out_real: &mut Vec<f64>,
) -> RetCode {
   let mut period_total: f64;
   let mut temp_real: f64;
   let mut i: i32;
   let mut out_idx: i32;
   let mut trailing_idx: i32;
   let lookback_total: i32;
   lookback_total = opt_in_time_period - 1;
   if start_idx < lookback_total {
      start_idx = lookback_total;
   }
   if start_idx > end_idx {
      *out_beg_idx = 0;
      *out_nb_element = 0;
      return RetCode::Success;
   }
   period_total = 0f64;
   trailing_idx = start_idx - lookback_total;
   i = trailing_idx;
   if opt_in_time_period > 1 {
      while i < start_idx {
         period_total += in_real[i as usize];
         i += 1;
      }
   }
   out_idx = 0;
   loop {
      period_total += in_real[i as usize];
      i += 1;
      temp_real = period_total;
      period_total -= in_real[trailing_idx as usize];
      trailing_idx += 1;
      //out_real[out_idx as usize] = temp_real / opt_in_time_period as f64;
      out_real.push(temp_real / opt_in_time_period as f64);
      out_idx += 1;
      if i > end_idx {
         break;
      }
   }
   *out_nb_element = out_idx;
   *out_beg_idx = start_idx;
   return RetCode::Success;
}

#[cfg(test)]
mod tests {
   #[test]
   fn it_works() {
      assert_eq!(2 + 2, 4);
   }
}
