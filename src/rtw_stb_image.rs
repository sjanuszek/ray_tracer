use core::slice;
use std::{alloc::{Layout, alloc, dealloc}, env::current_dir, ffi::CString, mem::MaybeUninit, ptr::{null, null_mut}};

use stb_image::{image::load, stb_image::{stbi_image_free, stbi_load, stbi_loadf}};

pub struct RtwImage {
    bytes_per_pixel: i32,
    fdata: *mut f32,
    bdata: *mut u8,
    image_width: i32,
    image_height: i32,
    bytes_per_scanline: i32,
}

unsafe impl Send for RtwImage {}
unsafe impl Sync for RtwImage {}

impl Default for RtwImage {
    fn default() -> Self {
        Self {
            bytes_per_pixel: 3,
            fdata: null_mut(),
            bdata: null_mut(),
            image_width: 0,
            image_height: 0,
            bytes_per_scanline: 0
        }
    }
}

impl RtwImage {
    pub fn new(image_filename: &str) -> Self {
        let mut image = RtwImage::default();
        let imagedir = current_dir().unwrap();
        
        let file_path = format!("{}/{}", imagedir.to_string_lossy(), image_filename);
        let c_str = CString::new(file_path.clone()).expect("Failed to create CString");

        if !image.load(c_str.as_ptr()) {
            panic!("Could not load file {}", file_path)
        }

        image
    }

    pub fn load(&mut self, filename: *const i8) -> bool {
        let mut n = self.bytes_per_pixel;
        unsafe {
            self.fdata = stbi_loadf(filename, &mut self.image_width, &mut self.image_height, &mut n, self.bytes_per_pixel);
        }

        if self.fdata.is_null() {
            return false;
        }

        self.bytes_per_scanline = self.image_width * self.bytes_per_pixel;
        self.convert_to_bytes();
        true
    }

    pub fn width(&self) -> i32 {
        if self.fdata.is_null() {
            0
        } else {
            self.image_width
        }
    }

    pub fn height(&self) -> i32 {
        if self.fdata.is_null() {
            0
        } else {
            self.image_height
        }
    }

    pub fn pixel_data(&self, mut x: i32, mut y: i32) -> &[u8]{
        static MAGENTA: [u8; 3] = [255, 0, 255];

        if self.bdata.is_null() {
            return &MAGENTA;
        }

        x = x.clamp(0, self.image_width - 1);
        y = y.clamp(0, self.image_height - 1);

        let offset = (y * self.bytes_per_scanline + x * self.bytes_per_pixel) as usize;

        unsafe {
            slice::from_raw_parts(self.bdata.add(offset), self.bytes_per_pixel as usize)
        }
    }

    fn float_to_byte(value: f32) -> u8 {
        if value <= 0. {
            return 0;
        } else if 1.0 <= value {
            return 255;
        }
        return (256. * value) as u8;
    }

    fn convert_to_bytes(&mut self) {
        let total_bytes = (self.image_width * self.image_height * self.bytes_per_pixel) as usize;

        let layout = Layout::array::<u8>(total_bytes).unwrap();

        self.bdata = unsafe {
            alloc(layout)
        };

        unsafe {
            let f_slice = slice::from_raw_parts(self.fdata, total_bytes);
            let b_slice = slice::from_raw_parts_mut(self.bdata, total_bytes);

            for i in 0..total_bytes {
                b_slice[i] = Self::float_to_byte(f_slice[i]);
            }
        }
    }
} 

impl Drop for RtwImage {
    fn drop(&mut self) {
        unsafe {
            if !self.fdata.is_null() {
                stbi_image_free(self.fdata as *mut _);
            }

            if !self.bdata.is_null() {
                let total_bytes = (self.image_width * self.image_height * self.bytes_per_pixel) as usize;
                let layout = Layout::array::<u8>(total_bytes).unwrap();
                dealloc(self.bdata, layout);
            }
        }
    }
}
