pub fn is_leap_year(year: u64) -> bool {
 // A leap year (in the Gregorian calendar) occurs:

 // - In every year that is evenly divisible by 4.
 // - Unless the year is evenly divisible by 100, in which case it's only a leap year if the year is also evenly divisible by 400.
 if year % 4 == 0 {
     // continue check
     if year % 100 == 0 { // if the year is divisible
         if year % 400 == 0 { // we perform one last check to see if that is divisible by 400 if
                              // YES return true
             return true;
         }
         else {
             return false;
         }
     }
     return true;
 }
    else {
       false
    }
}
