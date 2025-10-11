const std = @import("std");

pub fn print_answers(day: u32, part1: u64, part2: u64) void {
    std.log.info("Day {}", .{day});
    std.log.info("\tPart 1: {}", .{part1});
    std.log.info("\tPart 2: {}", .{part2});
}
