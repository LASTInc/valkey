
use std::ptr::null;

use crate::{zfree, zmalloc, zrelloc};
use crate::zmalloc::{valkey_malloc, valkey_realloc, valkey_free};

#[repr(C)]
struct vector {
    data: * mut (),      // Pointer to the actual array data
    alloc: u32,   // Number of allocated items
    len: u32,     // Current number of used items
    item_size: usize, // Size of each element in bytes
}


/* Usage example:
 *   vector arr;
 *   vectorInit(&arr, 10, sizeof(int));  // Initialize for 10 integers
 *
 *   int* new_int = vectorPush(&arr);    // Add new element
 *   *new_int = 42;                      // Initialize value
 *
 *   int* val = vectorGet(&arr, 0);      // Access element
 *   printf("%d\n", *val);               // Output: 42
 *
 *   vectorCleanup(&arr);                // Release memory
 */

/* Initialize a dynamic array (vector). */
#[unsafe(no_mangle)]
fn vectorInit(a: * mut vector, alloc: u32, item_size: usize) {
    if alloc != 0 {
        unsafe {
            (*a).data = zmalloc!((alloc as usize) * item_size);
        }
    } else {
        unsafe {
        (*a).data = 0 as * mut _ ;
        }
    }
    unsafe {
        (*a).alloc = alloc;
        (*a).len = 0;
        (*a).item_size = item_size;
    }
}



/* Get current vector length. */
#[unsafe(no_mangle)]
fn vectorLen(a: *mut vector) -> u32{
    unsafe {
        (*a).len
    } 
}



#[unsafe(no_mangle)]
fn vectorGet(a: *mut vector, idx: u32) -> *mut () {
    let new_ptr = unsafe {(*a).data as * mut _ as usize};
    unsafe {
        (new_ptr + ((idx as usize)*(*a).item_size)) as *mut ()
    }
}


#[unsafe(no_mangle)]
fn vectorPush(a: * mut vector) -> *mut (){
    unsafe {
        if (*a).len == (*a).alloc {
            let new_alloc = if (*a).alloc != 0{
                (*a).alloc * 2
            } else {
                8
            };
            (*a).data = zrelloc!((*a).data, new_alloc as usize);
            (*a).alloc = new_alloc;
        }
        let ptr = (*a).data as usize + (*a).item_size*((*a).len as usize);
        (*a).len += 1;
        ptr as * mut ()
    }
}


#[unsafe(no_mangle)]
fn vectorCleanup(a: *mut vector) {
    unsafe {
    if (*a).data as usize != 0 {
        zfree!((*a).data);
    }
    }
}