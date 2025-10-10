const std = @import("std");

const day1 = @import("day1.zig");

const day1_input = @embedFile("../input/2024/day1.txt");

pub fn run_all() !void {
    std.log.info("Running all solutions for 2024", .{});

    try day1.run(day1_input);
}

test {
    _ = day1;
}
