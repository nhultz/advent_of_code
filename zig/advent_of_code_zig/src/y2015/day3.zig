const std = @import("std");
const fs = std.fs;
const mem = std.mem;
const testing = std.testing;

pub fn run(input_file: []const u8) anyerror!void {
    var input = try std.fs.cwd().openFile(input_file, .{});
    defer input.close();

    var buf: [1024 * 1024]u8 = undefined;
    const bytes_read = try input.readAll(&buf);

    const allocator = std.heap.page_allocator;

    const answer_part1: u32 = try part1(buf[0..bytes_read], allocator);
    const answer_part2: u32 = try part2(buf[0..bytes_read], allocator);

    std.debug.print("Day 3\n", .{});
    std.debug.print("  Part 1: {}\n", .{answer_part1});
    std.debug.print("  Part 2: {}\n\n", .{answer_part2});
}

fn part1(input: []const u8, alloc: mem.Allocator) !u32 {
    var houses = std.AutoHashMap(House, u32).init(alloc);
    defer houses.deinit();

    var x: i32 = 0;
    var y: i32 = 0;
    try increment_count(&houses, x, y);

    for (input) |direction| {
        switch (direction) {
            '<' => x -= 1,
            '^' => y += 1,
            '>' => x += 1,
            'v' => y -= 1,
            else => {},
        }

        try increment_count(&houses, x, y);
    }

    return houses.count();
}

test "part1" {
    const test_allocator = testing.allocator;
    try testing.expect((try part1(">", test_allocator)) == 2);
    try testing.expect((try part1("^>v<", test_allocator)) == 4);
    try testing.expect((try part1("^v^v^v^v^v", test_allocator)) == 2);
}

fn part2(input: []const u8, alloc: mem.Allocator) !u32 {
    var houses = std.AutoHashMap(House, u32).init(alloc);
    defer houses.deinit();

    var x: i32 = 0;
    var y: i32 = 0;
    var robo_x: i32 = 0;
    var robo_y: i32 = 0;
    var santa_turn = true;
    try increment_count(&houses, x, y);

    for (input) |direction| {
        const cur_x = if (santa_turn) &x else &robo_x;
        const cur_y = if (santa_turn) &y else &robo_y;

        switch (direction) {
            '<' => cur_x.* -= 1,
            '>' => cur_x.* += 1,
            '^' => cur_y.* += 1,
            'v' => cur_y.* -= 1,
            else => {},
        }

        santa_turn = !santa_turn;
        try increment_count(&houses, cur_x.*, cur_y.*);
    }

    return houses.count();
}

test "part2" {
    const test_allocator = testing.allocator;
    try testing.expect((try part2("^v", test_allocator)) == 3);
    try testing.expect((try part2("^>v<", test_allocator)) == 3);
    try testing.expect((try part2("^v^v^v^v^v", test_allocator)) == 11);
}

const House = struct { x: i32, y: i32 };

fn increment_count(houses: *std.AutoHashMap(House, u32), x: i32, y: i32) !void {
    const coord = House{ .x = x, .y = y };

    var count: u32 = undefined;
    if (houses.contains(coord)) {
        const current_val = houses.get(coord).?;
        count = current_val + 1;
    } else {
        count = 1;
    }

    try houses.put(coord, count);
}
