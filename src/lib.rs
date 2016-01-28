#![crate_name = "file_limit"]
#![cfg(linux)]

extern crate libc;

use std::io::{Error, Result};

// private fn to get the kernal struct for file limits
#[inline]
fn get_limit() -> Result<libc::rlimit> {
    unsafe {
        let mut rlim = libc::rlimit{rlim_cur: 0, rlim_max: 0}; 
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlim) != 0 {
            return Err(Error::last_os_error());
        }
        return Ok(rlim)
    }
}

#[inline]
pub fn get() -> Result<usize> {
    match get_limit() {
        Ok(rlim) => Ok(rlim.rlim_cur as usize), 
        Err(err) => Err(err),
    }
}

pub enum MaxLimit {
    Val(usize),
    NoLimit,
}

#[inline]
pub fn max() -> Result<MaxLimit> {
    let rlim = match get_limit() {
        Ok(rlim) => rlim,
        Err(err) => return Err(err),
    };
    if rlim.rlim_max == libc::RLIM_INFINITY {
        Ok( MaxLimit::NoLimit )
    } else {
        Ok( MaxLimit::Val(rlim.rlim_max as usize))
    }
}

#[inline]
pub fn set_to_max() -> Result<usize> {
    let mut r = match get_limit() {
        Ok(l) => l,
        Err(err) => return Err(err),
    };
    if r.rlim_max != r.rlim_cur {
        if r.rlim_max == libc::RLIM_INFINITY { // no limit restriction
            r.rlim_cur *= 8;
        } else if r.rlim_cur < r.rlim_max{
            r.rlim_cur = r.rlim_max;
        }
        unsafe {
            if libc::setrlimit(libc::RLIMIT_NOFILE, &r) < 0 {
                let err = Error::last_os_error();
                return Err(err);
            }
        }
    }

    Ok(r.rlim_cur as usize)
}

#[test]
fn get_file_limit() {
    let lim = get().unwrap();
    assert!(lim > 0);
}

#[test]
fn max_limit() {
    match max() {
        Ok(MaxLimit::Val(v)) => {
            assert!(v > 0);
        },
        Ok(MaxLimit::NoLimit) => assert!(true),
        Err(err) => assert!(false),
    }
}

#[test]
fn set_max() {
    let cur_limit: usize = get().unwrap();
    let max_limit = set_to_max().unwrap();
    let new_limit: usize = get().unwrap();
    assert_eq!(max_limit, new_limit);
    assert!(new_limit >= cur_limit);
}
