use std::io::prelude::*;
use std::{mem, slice, io::IoSlice};

pub struct BMP {
    bit_map_file_header: BitMapFileHeader,
    bit_map_core_header: BitMapCoreHeader,
    color_table: ColorTable,
    bit_map_array: BitMapArray,
    width: u16,
    height: u16,
}

impl BMP {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        let height: i32 = data.len() as i32;
        let width: i32 = if height > 0 { data[0].len() as i32 } else { 0 };
        assert!(height < 65535);
        assert!(width < 65535);
        let offset_bits: i32 = 14 + 12 + 256 * 3;
        let file_size: i32 = offset_bits + width * height;
        Self {
            bit_map_file_header: BitMapFileHeader::new(file_size, offset_bits),
            bit_map_core_header: BitMapCoreHeader::new(width as u16, height as u16),
            color_table: ColorTable::new(),
            bit_map_array: BitMapArray::new(data),
            width: width as u16,
            height: height as u16,
        }
    }

    pub fn push_color(&mut self, color: RGBTRIPLE) -> Result<u8, &'static str> {
        self.color_table.push_color(color)
    }

    pub fn insert_color(&mut self, color: RGBTRIPLE, ind: u8) {
        self.color_table.insert_color(color, ind);
    }
}

impl BMP {
    pub fn write_to_file(&self, file: &mut std::fs::File) {
        let mut file = std::io::BufWriter::new(file);
        println!("start to write file");
        self.bit_map_file_header.write_to_file(&mut file);
        self.bit_map_core_header.write_to_file(&mut file);
        self.color_table.write_to_file(&mut file);
        self.bit_map_array.write_to_file(&mut file);
        println!("all right");
    }
}

struct BitMapFileHeader {
    _type: u16,
    _size: i32,
    _reserved1: u16,
    _reserved2: u16,
    _offsetbits: i32,
}

impl BitMapFileHeader {
    fn new(file_size: i32, offset_bits: i32) -> Self {
        Self {
            _type: 0x4D42,
            _size: file_size,
            _reserved1: 0,
            _reserved2: 0,
            _offsetbits: offset_bits,
        }
    }
}

impl BitMapFileHeader {
    fn write_to_file(&self, file: &mut std::io::BufWriter<&mut std::fs::File>) {
        println!("start to file header");
        print!("file with size: {} ", self._size);
        let a1 = self._type.to_le_bytes();
        let a2 = self._size.to_le_bytes();
        let a3 = self._reserved1.to_le_bytes();
        let a4 = self._reserved2.to_le_bytes();
        let a5 = self._offsetbits.to_le_bytes();
        let a = [
            IoSlice::new(&a1),
            IoSlice::new(&a2),
            IoSlice::new(&a3),
            IoSlice::new(&a4),
            IoSlice::new(&a5)
        ];
        file.write_vectored(&a).unwrap();
    }
}

struct BitMapCoreHeader {
    _size: u32,
    _width: u16,
    _height: u16,
    _planes: u16,
    _bitcount: u16,
}

impl BitMapCoreHeader {
    fn new(width: u16, height: u16) -> Self {
        Self {
            _size: 12,
            _width: width,
            _height: height,
            _planes: 1,
            _bitcount: 8,
        }
    }
}

impl BitMapCoreHeader {
    fn write_to_file(&self, file: &mut std::io::BufWriter<&mut std::fs::File>) {
        println!("start to core header");
        file.write_vectored(&[
            IoSlice::new(&self._size.to_le_bytes()),
            IoSlice::new(&self._width.to_le_bytes()),
            IoSlice::new(&self._height.to_le_bytes()),
            IoSlice::new(&self._planes.to_le_bytes()),
            IoSlice::new(&self._bitcount.to_le_bytes()),
            ]).unwrap();
    }
}

pub struct RGBTRIPLE {
    rgbtRed: u8,
    rgbtGreen: u8,
    rgbtBlue: u8,
}

impl RGBTRIPLE {
    pub const DEFAULT: Self = Self::new(0, 0, 0);

    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            rgbtRed: red,
            rgbtGreen: green,
            rgbtBlue: blue,
        }
    }
}

impl RGBTRIPLE {
    fn write_to_file(&self, file: &mut std::io::BufWriter<&mut std::fs::File>) {
        file.write_vectored(&[IoSlice::new(&[self.rgbtBlue, self.rgbtGreen, self.rgbtRed])]).unwrap();
    }
}

struct ColorTable {
    table: [RGBTRIPLE; 256], // 256 by 4 baits
    write_ind: u8,
}

impl ColorTable {
    fn new() -> Self {
        Self {
            table: [RGBTRIPLE::DEFAULT; 256],
            write_ind: 0,
        }
    }

    fn push_color(&mut self, color: RGBTRIPLE) -> Result<u8, &'static str> {
        if self.write_ind < 255 {
            self.table[self.write_ind as usize] = color;
            self.write_ind += 1;
            Ok(self.write_ind - 1)
        } else {
            Err("cannot push color")
        }
    }

    fn insert_color(&mut self, color: RGBTRIPLE, ind: u8) {
        self.table[ind as usize] = color;
    }
}

impl ColorTable {
    fn write_to_file(&self, file: &mut std::io::BufWriter<&mut std::fs::File>) {
        println!("start to color table");
        self.table.iter().for_each(|rgb| rgb.write_to_file(file));
    }
}

struct BitMapArray {
    buf: Vec<Vec<u8>>,
}

impl BitMapArray {
    fn new(data: Vec<Vec<u8>>) -> Self {
        Self { buf: data }
    }
}

impl BitMapArray {
    fn write_to_file(&self, file: &mut std::io::BufWriter<&mut std::fs::File>) {
        println!("start to map array");
        for row in &self.buf {
            file.write_vectored(&[IoSlice::new(&row[..])]).unwrap();
        }
    }
}

pub fn to_u8<T: Copy + Sized>(s: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(s.as_ptr() as *const u8, s.len() * mem::size_of::<T>()) }
}
