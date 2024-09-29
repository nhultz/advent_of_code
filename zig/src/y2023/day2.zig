const std = @import("std");

pub fn run(input: []const u8) !void {
    const answer_part1: u64 = try part1(input);
    const answer_part2: u64 = try part2(input);

    std.debug.print("Day 2\n", .{});
    std.debug.print("  Part 1: {}\n", .{answer_part1});
    std.debug.print("  Part 2: {}\n\n", .{answer_part2});
}

fn part1(input: []const u8) !u64 {
    _ = input;
    return 0;
}

fn part2(input: []const u8) !u64 {
    _ = input;
    return 0;
}
