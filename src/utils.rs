use crate::BufferMap;

/// Determines the last buffer index required by this mapping.
/// This is not a fast function, use at your own risk!
pub fn get_buffer_length(buffer_map: &BufferMap) -> usize {
    let mut max_i: usize = 256;

    // Yes, there is a sexy way to do this, but I also want to go to bed.
    for (b, _) in buffer_map.iter() {
        if (*b as usize) < max_i {
            max_i = *b as usize;
        }
    }
    max_i
}

/// Returns a vec of used buffer indices.
pub fn get_used_buffers(buffer_map: &BufferMap) -> Vec<usize> {
    buffer_map.iter().map(|(i, _)| *i as usize).collect()
}