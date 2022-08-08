const std = @import("std");
const fs = std.fs;
const testing = std.testing;

pub fn run(input_file: []const u8) anyerror!void {
    var input = try std.fs.cwd().openFile(input_file, .{});
    defer input.close();

    var buf: [1024 * 1024]u8 = undefined;
    const bytes_read = try input.readAll(&buf);

    const answer_part1: i32 = part1(buf[0..bytes_read]);
    const answer_part2: usize = part2(buf[0..bytes_read]);

    std.debug.print("Day 1\n", .{});
    std.debug.print("  Part 1: {}\n", .{answer_part1});
    std.debug.print("  Part 2: {}\n\n", .{answer_part2});
}

fn part1(input: []const u8) i32 {
    var floor: i32 = 0;

    for (input) |c| {
        switch (c) {
            '(' => floor += 1,
            ')' => floor -= 1,
            else => {},
        }
    }

    return floor;
}

test "part1" {
    try testing.expect(part1("(())") == 0);
    try testing.expect(part1("(()(()(") == 3);
    try testing.expect(part1("))(") == -1);
    try testing.expect(part1(")))") == -3);
}

fn part2(input: []const u8) usize {
    var floor: i32 = 0;
    var position: usize = undefined;

    for (input) |c, idx| {
        switch (c) {
            '(' => floor += 1,
            ')' => floor -= 1,
            else => {},
        }

        if (floor == -1) {
            position = idx + 1;
            break;
        }
    }

    return position;
}

test "part2" {
    try testing.expect(part2(")") == 1);
    try testing.expect(part2("()())") == 5);
}
