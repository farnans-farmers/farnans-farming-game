use std::time::{Duration, Instant};

use sdl2::rect::Rect;

/// Looping frame animation.
///
/// Animations support two interchangeable modes of operation. The tick and
/// tick_at methods implement framerate-independent animation, useful for
/// things like sprites walking or wind effects. The next and advance methods
/// implement frame count animation, useful for things that increment between
/// linear states.
///
/// Most often, you'll want an Animation<Rect> to animate between positions on
/// a spritesheet. You might also want an Animation<Texture> to animate entire
/// tilemaps at once, e.g. for applying wind animations to all ground types.
pub struct Animation<T> {
    k: usize,
    frames: Vec<T>,
    dur: Duration,
    next_frame: Instant,
}

impl<T> Animation<T> {
    /// Creates an animation starting on the first element of frames. The
    /// animation advances a frame each time a tick time exceeds a multiple of
    /// frame_length after base.
    /// Panics if frames is empty or frame_length <= 0.
    pub fn new(frames: Vec<T>, frame_length: Duration, base: Instant) -> Animation<T> {
        if frames.is_empty() {
            panic!("can't animate zero frames")
        }
        if frame_length <= Duration::ZERO {
            panic!("frame duration must be positive")
        }
        let now = Instant::now();
        Animation {
            k: 0,
            frames: frames,
            dur: frame_length,
            next_frame: base + frame_length,
        }
    }

    /// Resets the animation to a new base time, returning to the first frame.
    /// Use this to switch between different animations for one character, or
    /// to base advance on the first frame. Returns self for chaining.
    pub fn reset(&mut self, when: Instant) -> &mut Animation<T> {
        self.k = 0;
        self.next_frame = when + self.dur;
        self
    }

    /// Computes the frame to use at the given time. Does nothing if
    /// when is earlier than the last tick.
    pub fn tick_at(&mut self, when: Instant) -> &T {
        if when < self.next_frame {
            return self.current();
        }
        let d = when.duration_since(self.next_frame).as_micros() as usize;
        let fp = self.dur.as_micros() as usize;
        let adv = d / fp + 1;
        let nk = (self.k + adv) % self.frames.len();
        self.k = nk as usize;
        let dt = adv * self.dur.as_micros() as usize;
        self.next_frame += Duration::from_micros(dt as u64);
        self.current()
    }

    // Computes the current frame.
    pub fn tick(&mut self) -> &T {
        self.tick_at(Instant::now())
    }

    /// Advances a number of frames regardless of time. The last frame time
    /// advances by the equivalent of n frames.
    pub fn advance(&mut self, n: usize) -> &T {
        self.k += n;
        self.k %= self.frames.len();
        let dt = (n as u128) * self.dur.as_micros();
        let dt = Duration::from_micros(dt as u64);
        self.next_frame += dt;
        self.current()
    }

    /// Moves to the next frame regardless of time.
    pub fn next(&mut self) -> &T {
        self.advance(1)
    }

    /// Borrows the current frame.
    pub fn current(&self) -> &T {
        &self.frames[self.k]
    }

    /// Returns the current frame index.
    pub fn current_index(&self) -> usize {
        self.k
    }
}

impl Animation<Rect> {
    /// Creates an animation dividing a row on a spritesheet into
    /// equal-size tiles.
    pub fn from_sheet(
        bounds: &Rect,
        row_top: i32,
        tile_width: u32,
        tile_height: u32,
        frame_length: Duration,
        base: Instant,
    ) -> Animation<Rect> {
        let n = bounds.width() / tile_width;
        Animation::n_from_sheet(row_top, tile_width, tile_height, n, frame_length, base)
    }

    /// Creates an animation with n equal-size tiles along a row in a
    /// spritesheet.
    pub fn n_from_sheet(
        row_top: i32,
        tile_width: u32,
        tile_height: u32,
        n: u32,
        frame_length: Duration,
        base: Instant,
    ) -> Animation<Rect> {
        let mut v = Vec::with_capacity(n as usize);
        for i in 0..n {
            v.push(Rect::new(
                (i * tile_width) as i32,
                row_top,
                tile_width,
                tile_height,
            ));
        }
        Animation::new(v, frame_length, base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uses_first_frame() {
        let v = vec![1, 2, 3];
        let anim = Animation::new(v, Duration::from_secs(1), Instant::now());
        assert_eq!(*anim.current(), 1);
    }

    #[test]
    fn next_iters() {
        let v = vec![1, 2, 3];
        let mut anim = Animation::new(v.clone(), Duration::from_secs(1), Instant::now());
        for _i in 0..100 {
            for k in v.iter() {
                let c = *anim.current();
                assert_eq!(*k, c);
                assert_ne!(c, *anim.next());
            }
        }
    }

    #[test]
    fn advance_many() {
        let v = vec![1, 2, 3];
        let mut anim = Animation::new(v.clone(), Duration::from_secs(1), Instant::now());
        for _i in 0..100 {
            // Too lazy to test expected results, so just check this succeeds.
            anim.advance(3);
            anim.advance(1000000);
        }
    }

    #[test]
    fn tick_at_advances() {
        let v = vec![1, 2, 3];
        let d = Duration::from_secs(1);
        let start = Instant::now();
        let mut anim = Animation::new(v.clone(), d, start);
        for _i in 0..100 {
            assert_eq!(*anim.tick_at(start), v[0]);
        }
        for i in 0..100 {
            let t = start + i * (d / 1000);
            assert_eq!(*anim.tick_at(t), v[0]);
        }
        struct Testcase {
            when: Duration,
            want: usize,
        }
        let cases = vec![
            Testcase {
                when: d - Duration::from_micros(1),
                want: 0,
            },
            Testcase { when: d, want: 1 },
            Testcase {
                when: 2 * d,
                want: 2,
            },
            Testcase {
                when: 3 * d,
                want: 0,
            },
            Testcase {
                when: 6 * d,
                want: 0,
            },
            Testcase {
                when: 7 * d,
                want: 1,
            },
            Testcase {
                when: 60 * d,
                want: 0,
            },
        ];
        for c in cases {
            let t = start + c.when;
            println!(
                "d={:?} t={:?} frame={:?} next={:?}",
                c.when, t, anim.k, anim.next_frame
            );
            let r = *anim.tick_at(t);
            assert_eq!(
                r, v[c.want],
                "wrong tick: d is {:?}, got {}, want {}",
                c.when, r, v[c.want]
            );
            assert_eq!(*anim.tick_at(start), v[c.want]);
        }
    }

	#[test]
	fn reset_resets() {
        let v = vec![1, 2, 3];
        let mut anim = Animation::new(v, Duration::from_secs(1), Instant::now());
		assert_eq!(*anim.next(), 2);
		assert_eq!(*anim.reset(Instant::now()).current(), 1);
	}

	#[test]
	fn from_sheet_full() {
		let bounds = Rect::new(0, 0, 3, 1);
		let mut anim = Animation::from_sheet(&bounds, 0, 1, 1, Duration::from_secs(1), Instant::now());
		let v = vec![Rect::new(0, 0, 1, 1), Rect::new(1, 0, 1, 1), Rect::new(2, 0, 1, 1)];
		for r in v {
			assert_eq!(r, *anim.current());
			anim.next();
		}
		assert_eq!(anim.current_index(), 0);
	}

	#[test]
	fn from_sheet_partial() {
		let bounds = Rect::new(0, 0, 5, 1);
		let mut anim = Animation::from_sheet(&bounds, 0, 2, 1, Duration::from_secs(1), Instant::now());
		let v = vec![Rect::new(0, 0, 2, 1), Rect::new(2, 0, 2, 1)];
		for r in v {
			assert_eq!(r, *anim.current());
			anim.next();
		}
		assert_eq!(anim.current_index(), 0);
	}

    #[test]
    #[should_panic]
    fn need_frames() {
        Animation::<i32>::new(Vec::new(), Duration::from_secs(1), Instant::now());
    }

    #[test]
    #[should_panic]
    fn need_positive_frame_length() {
        Animation::<i32>::new(vec![1, 2, 3], Duration::ZERO, Instant::now());
    }
}
