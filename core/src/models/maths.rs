pub fn add(a:f64, b:f64) -> f64 {
    a + b
}

pub fn variance(data: &[f64]) -> Option<f64> {
      let n = data.len();                                                                                                                          
      if n < 2 {                                                                                                                                 
          return None;                                                                                                                             
      }
      let mean = data.iter().sum::<f64>() / n as f64;                                                                                              
      let var = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;                                                           
      Some(var)                                                                                                                                    
  }
                                                                                                                                                   
pub fn standard_deviation(data: &[f64]) -> Option<f64> {                                                                                       
      variance(data).map(f64::sqrt)
  }
