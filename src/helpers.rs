use std::ffi::CString;

pub trait NullTerm {
    fn to_cstring_with_null(&self) -> CString;
}

impl NullTerm for String {
    fn to_cstring_with_null(&self) -> CString {
        CString::from_vec_with_nul({
            let mut a = self.clone().into_bytes();
            a.push(0);
            a
        }).unwrap()
    }
}

pub fn construct_array(args: &Vec<String>) -> Vec<CString> {
    let mut a = args
        .iter()
        .map(|a| a.to_cstring_with_null())
        .collect::<Vec<CString>>();
    a.push(CString::from_vec_with_nul(vec![0]).unwrap());
    return a
}