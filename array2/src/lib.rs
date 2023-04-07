/// Cameron Castillo and Daniel Diaz Implementation for Array2.

pub mod array2 {

    /// Array2 is a struct that comprises a one-dimensional vector of type T, along a width and
    /// height, and serves to emulate a two-dimensional array using one-dimensional space. The
    /// width represents the number of columns in the array, while the height represent the number
    /// of rows.
    ///
    /// # Examples
    ///
    /// This will allocate an array2 containing one-dimensional vector of some_data of width x and
    /// height y:
    ///
    /// ```
    ///
    /// use array2::array2::Array2;
    /// let array: Array2<i32>= Array2::new();
    ///
    /// ```
    ///
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Array2<T: Clone> {
        pub data: Vec<T>,
        width: usize,
        height: usize,
    }

    impl<T: Clone> Array2<T> {
        /// ## Iterates over the Array2 in row-major order.
        ///
        /// When iterating over Array2 in row-major order, the data is read from
        /// left to right until the upper-bounding row is reached.
        ///
        /// # Row-major Formula
        /// [width * row + col]
        ///
        /// # Examples
        /// ```
        ///
        /// use array2::array2::Array2;
        /// let data = vec![1, 2, 3, 4];
        /// let array = Array2::from_row_major(2, 2, data);
        ///
        /// ```
        pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
            (0..self.height)
                .flat_map(move |r| (0..self.width).map(move |c| (c, r, self.get(c, r).unwrap())))
        }

        /// ## Iterates over the Array2 in column-major order.
        ///
        /// When iterating over Array2 in column-major order, the data is read from top to
        /// bottom until the upper-bounding column is reached.
        ///
        /// # Column-major Formula
        /// [height * col + row]
        ///
        /// # Examples
        /// ```
        ///
        /// use array2::array2::Array2;
        /// let data = vec![1, 2, 3, 4];
        /// let array = Array2::from_col_major(2, 2, data);
        ///
        /// ```
        pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
            (0..self.width)
                .map(move |c| (c, self.data.iter().skip(c)))
                .flat_map(move |(c, col)| {
                    col.step_by(self.width)
                        .enumerate()
                        .map(move |(r, val)| (c, r, val))
                })
        }

        /// ## Construct a new instance of array2.
        ///
        /// The array2 is initially created with empty an empty vector, width 0, and height 0 .
        /// # Examples
        /// ```
        /// use array2::array2::Array2;;
        ///
        /// let mut array: Array2<i32> = Array2::new();
        ///
        /// ```
        pub fn new() -> Self {
            Self {
                data: Vec::new(),
                width: 0,
                height: 0,
            }
        }

        /// ## Construct a new instance of Array2 storing the data in row major order.
        ///
        /// The array2 width and height must be specified in this constructor, and the Vector data will
        /// be stored in row-major order.
        ///
        /// # Examples
        /// ```
        ///
        /// use array2::array2::Array2;
        ///
        /// let data = vec![1, 2, 3, 4];
        /// let array = Array2::from_row_major(2, 2, data);
        ///
        /// ```
        pub fn from_row_major(width: usize, height: usize, orig_vec: Vec<T>) -> Self {
            Self {
                data: Vec::from(orig_vec),
                width: width,
                height: height,
            }
        }

        /// ## Construct a new instance of Array2 storing the data in column major order.
        ///
        /// The array2 width and height must be specified in this constructor, and the Vector data will
        /// be stored in column-major order.
        ///
        /// # Examples
        ///  ```
        ///
        /// use array2::array2::Array2;
        ///
        /// let data = vec![1, 2, 3, 4];
        /// let array = Array2::from_col_major(2, 2, data);
        ///
        ///  ```
        pub fn from_col_major(width: usize, height: usize, orig_vec: Vec<T>) -> Self {
            let mut temp = Vec::new();
            for c in 0..width.clone() {
                let x: Vec<T> = orig_vec
                    .clone()
                    .into_iter()
                    .skip(c)
                    .step_by(width.clone())
                    .collect();
                for val in x.into_iter() {
                    temp.push(val)
                }
            }
            Self {
                data: Vec::from(temp),
                width,
                height,
            }
        }

        /// ## Returns value at the given row column
        ///
        /// This function return the current value T located inside Vec<T> at position
        /// row column in array2.
        ///
        /// # Example
        /// ```
        ///
        /// use array2::array2::Array2;
        /// let data = vec![1, 2, 3, 4];
        /// let array = Array2::from_col_major(2, 2, data);
        ///
        /// ```
        pub fn get(&self, c: usize, r: usize) -> Option<&T> {
            self.get_index(c, r).map(|index| &self.data[index])
        }

        /// ## Returns a mutable reference to the value at the given row colum
        ///
        /// This function returns mutable reference to the T value inside Vec<T> at position
        /// row column in array2.
        ///
        /// # Example
        /// ```
        ///
        /// use array2::array2::Array2;
        /// let data = vec![1, 2, 3, 4];
        /// let mut array = Array2::from_col_major(2, 2, data);
        /// let x = 30;
        /// if let Some(y) = array.get_mut(0, 1){
        ///     *y = 30
        /// }
        ///
        ///  ```
        pub fn get_mut(&mut self, c: usize, r: usize) -> Option<&mut T> {
            self.get_index(c, r).map(move |index| &mut self.data[index])
        }

        /// ## Gets the index of the specify row column under the 1d underlying Vec
        fn get_index(&self, c: usize, r: usize) -> Option<usize> {
            if c < self.width && r < self.height {
                Some(r * self.width + c)
            } else {
                None
            }
        }

        /// ## Returns the size of 1d Vec inside Array2
        ///
        /// This function returns the size of 1d Vec underlying the Array2 struct as usize.
        ///
        /// # Example
        /// ```
        ///
        /// use array2::array2::Array2;
        /// let data = vec![1, 2, 3, 4];
        /// let array = Array2::from_col_major(2, 2, data);
        ///
        /// ```
        pub fn size(&self) -> usize {
            self.width * self.height
        }

        /// ## Returns the height of Array2
        ///
        /// This function returns the height of the Array2 struct as usize.
        ///
        /// # Example
        /// ```
        ///
        /// use array2::array2::Array2;
        /// let data = vec![1, 2, 3, 4];
        /// let array = Array2::from_col_major(2, 2, data);
        ///
        /// ```
        pub fn get_height(&self) -> usize {
            self.height
        }

        /// ## Returns the width of Array2
        ///
        /// This function return the width of the Array2 struct as usize.
        ///
        /// # Example
        /// ```
        ///
        /// use array2::array2::Array2;
        /// let data = vec![1, 2, 3, 4];
        /// let array = Array2::from_col_major(2, 2, data);
        ///
        /// ```
        pub fn get_width(&self) -> usize {
            self.width
        }

        /// # Sets a given width and height to the current Array2.
        pub fn set_dimensions(&mut self, width: usize, height: usize) {
            self.width = width;
            self.height = height;
        }

        /// # Returns an Array2 that construct only a perfect square 2D matrix.
        ///
        /// The array2 width and height must be specified in this constructor, and the Vector data will
        /// be stored in row-major order. If either the width or height is odd, the dimension will
        /// be decreased by 1 to make it even.
        ///
        /// # Examples
        ///  ```
        ///
        /// use array2::array2::Array2;
        ///
        /// let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        /// let array = Array2::from_even_dimension(3, 3, data);
        /// assert_eq!(array.data, vec![1, 2, 4, 5]);
        ///
        ///  ```
        pub fn from_even_dimension(width: usize, height: usize, orig_vec: Vec<T>) -> Self {
            let mut trim_width: bool = false;
            let mut trim_height: bool = false;
            let mut new_width = width;
            let mut new_height = height;
            if width % 2 == 1 {
                trim_width = true;
                new_width -= 1;
            }
            if height % 2 == 1 {
                trim_height = true;
                new_height -= 1;
            }

            let mut temp = Vec::new();
            for r in 0..height {
                if r == height - 1 && trim_height {
                    continue;
                }
                for c in 0..width {
                    if c == width - 1 && trim_width {
                        continue;
                    }
                    let idx = r * width + c;
                    temp.push(orig_vec[idx].clone());
                }
            }

            Self {
                data: Vec::from(temp),
                width: new_width,
                height: new_height,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::array2::Array2;

    #[test]
    fn check() {
        use array2::Array2;
        let x = vec![1, 3, 2, 4];
        let y = Array2::from_col_major(2, 2, vec![1, 2, 3, 4]);
        assert_eq!(x, y.data);
    }

    #[test]
    fn test_trim() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let array = Array2::from_even_dimension(5, 3, data);
        assert_eq!(array.data, vec![1, 2, 3, 4, 6, 7, 8, 9]);
        let odd_width = vec![1, 2, 3, 4, 5, 6];
        let array1 = Array2::from_even_dimension(3, 2, odd_width);
        assert_eq!(array1.data, vec![1, 2, 4, 5]);
        let odd_height = vec![1, 2, 3, 4, 5, 6];
        let array2 = Array2::from_even_dimension(2, 3, odd_height);
        assert_eq!(array2.data, vec![1, 2, 3, 4]);
        let even_data = vec![1, 2, 3, 4];
        let array3 = Array2::from_even_dimension(2, 2, even_data.clone());
        assert_eq!(array3.data, even_data);
    }
}
