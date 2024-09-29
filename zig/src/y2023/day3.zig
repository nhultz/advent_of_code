const std = @import("std");

pub fn run(input: []const u8) !void {
    const answer_part1: u64 = try part1(input);
    const answer_part2: u64 = try part2(input);

    std.debug.print("Day 3\n", .{});
    std.debug.print("  Part 1: {}\n", .{answer_part1});
    std.debug.print("  Part 2: {}\n\n", .{answer_part2});
}

fn part1(input: []const u8) !u64 {
    const allocator = std.heap.page_allocator;

    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    var symbols = std.AutoHashMap(Point, u8).init(allocator);
    defer symbols.deinit();

    var row: usize = 0;
    var col: usize = 0;

    while (lines.next()) |line| {
        var num: u32 = 0;
        for (line) |c| {
            switch (c) {
                '0'...'9' => num = (num * 10) + (c - 48),
                '.' => {
                    std.debug.print("{c} ", .{c});
                    if (num > 0) {
                        std.debug.print("{} ", .{num});
                        num = 0;
                    }
                },
                '!', '@', '#', '$', '%', '&', '*', '/', '+', '-', '=' => {
                    try symbols.put(.{ .x = row, .y = col }, c);
                },
                else => {
                    std.debug.print("\nUnknown symbol: {c}\n", .{c});
                },
            }
            col += 1;
        }
        if (num > 0) {
            std.debug.print("{}", .{num});
            num = 0;
        }
        std.debug.print("\n\n", .{});
        row += 1;
    }

    return 0;
}

fn part2(input: []const u8) !u64 {
    _ = input;
    return 0;
}

const Point = struct { x: u64, y: u64 };
