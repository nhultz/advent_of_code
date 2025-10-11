const std = @import("std");
const testing = std.testing;

const util = @import("../util.zig");

pub fn run(allocator: std.mem.Allocator, input: []const u8) !void {
    const answer_part1: u64 = try part1(allocator, input);
    const answer_part2: u64 = try part2(allocator, input);
    util.print_answers(1, answer_part1, answer_part2);
}

fn part1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    var left_nums: std.ArrayList(i32) = .empty;
    defer left_nums.deinit(allocator);

    var right_nums: std.ArrayList(i32) = .empty;
    defer right_nums.deinit(allocator);

    while (lines.next()) |line| {
        var it = std.mem.splitSequence(u8, line, "   ");

        const n1 = try std.fmt.parseInt(i32, it.next().?, 10);
        try left_nums.append(allocator, n1);

        const n2 = try std.fmt.parseInt(i32, it.next().?, 10);
        try right_nums.append(allocator, n2);
    }

    std.mem.sort(i32, left_nums.items, {}, std.sort.asc(i32));
    std.mem.sort(i32, right_nums.items, {}, std.sort.asc(i32));

    var sum: u64 = 0;
    for (left_nums.items, right_nums.items) |left, right| {
        const n = @abs(left - right);
        sum += n;
    }

    return sum;
}

fn part2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    var left_nums: std.ArrayList(u32) = .empty;
    defer left_nums.deinit(allocator);

    var right_counts = std.AutoHashMap(u32, u32).init(allocator);
    defer right_counts.deinit();

    while (lines.next()) |line| {
        var it = std.mem.splitSequence(u8, line, "   ");

        const n1 = try std.fmt.parseInt(u32, it.next().?, 10);
        try left_nums.append(allocator, n1);

        const n2 = try std.fmt.parseInt(u32, it.next().?, 10);
        try right_counts.put(n2, (right_counts.get(n2) orelse 0) + 1);
    }

    var sum: u64 = 0;
    for (left_nums.items) |n| {
        const freq = right_counts.get(n) orelse 0;
        sum += (n * freq);
    }

    return sum;
}

test "part1" {
    const test_input =
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
    ;
    const ans = try part1(std.testing.allocator, test_input);
    try testing.expectEqual(11, ans);
}

test "part2" {
    const test_input =
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
    ;
    const ans = try part2(std.testing.allocator, test_input);
    try testing.expectEqual(31, ans);
}
