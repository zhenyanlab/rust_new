pub fn plus() -> i32{
   if 1==1{
        -1;
   } else{
    -2;
   }
   return -3;
}
pub fn find_store(mobile_os : &str) ->Option<&str>{
   match mobile_os {
       "IOS" => Some("app store"),
       "android" => Some("google play"),
       _ => None
   }
}



