use cgmath::Matrix4;

pub trait Mat4ToSliceExt<T> {
    fn as_slice(&self) -> &[T];
}

impl<T> Mat4ToSliceExt<T> for Matrix4<T> {
    fn as_slice(&self) -> &[T] {
        let slice: &[T; 16] = unsafe { std::mem::transmute(self) };
        slice
    }
}
