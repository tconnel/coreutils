// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use divan::{Bencher, black_box};
use uu_yes::uumain;
use uucore::benchmark::run_util_function;

/// Benchmark numbering many lines (default mode - most common use case)
#[divan::bench(args = [100_000])]
fn yes_many_lines(bencher: Bencher, _num_lines: usize) {
    bencher.bench(|| {
        black_box(run_util_function(uumain, &[]));
    });
}

fn main() {
    divan::main();
}
