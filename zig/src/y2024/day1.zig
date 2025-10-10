const std = @import("std");

pub fn run(input: []const u8) !void {
    const answer_part1: u64 = try part1(input);
    const answer_part2: u64 = try part2(input);

    std.log.info("Day 1", .{});
    std.log.info("  Part 1: {}", .{answer_part1});
    std.log.info("  Part 2: {}", .{answer_part2});
}

fn part1(input: []const u8) !u64 {
    _ = input;
    return 0;
}

fn part2(input: []const u8) !u64 {
    _ = input;
    return 0;
}
