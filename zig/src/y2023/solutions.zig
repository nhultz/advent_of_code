const day1 = @import("day1.zig");
// const day2 = @import("day2.zig");
// const day3 = @import("day3.zig");

const day1_input = @embedFile("../input/2023/day1.txt");
// const day2_input = @embedFile("../input/2023/day2.txt");
// const day3_input = @embedFile("../input/2023/day3.txt");

pub fn run_all() !void {
    try day1.run(day1_input);
    // try day2.run(day2_input);
    // try day3.run(day3_input);
}

test {
    _ = day1;
    // _ = day2;
    // _ = day3;
}
