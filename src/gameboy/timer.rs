use super::memory::Memory;

pub struct Timer {
    pub internal_divider: usize,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            internal_divider: 0,
        }
    }

    pub fn increment_div(&mut self, mem_map: &mut Memory, clock_cycles: usize) {
        self.internal_divider += clock_cycles as usize;
        if self.internal_divider >= 256 {
            self.internal_divider -= 256;
            // Increment div and check for overflow
            mem_map.div += 1;
            mem_map.div &= 0xFF;
        }
    }

    pub fn increment_tima(&mut self, mem_map: &mut Memory, clock_cycles: usize) -> bool {
        let timer_enabled = mem_map.tac & 0b100 != 0;
        if !timer_enabled {
            return false;
        }
        let input_clock_select = mem_map.tac & 0b0011;
        let clock_divider = match input_clock_select {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => unreachable!(),
        };

        // TODO: Check if this is correct
        mem_map.tima += clock_cycles / clock_divider;
        if mem_map.tima > 0xFF {
            mem_map.tima = mem_map.tma as usize;
            return true;
        }
        false
    }
}
