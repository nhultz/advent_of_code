const std = @import("std");
const fs = std.fs;
const io = std.io;
const mem = std.mem;
const testing = std.testing;

const mecha = @import("mecha");

pub fn run(input_file: []const u8) anyerror!void {
    var input = try fs.cwd().openFile(input_file, .{});
    defer input.close();

    var buf_reader = io.bufferedReader(input.reader());
    var in_stream = buf_reader.reader();

    const allocator = std.heap.page_allocator;

    var buf: [1024 * 1024]u8 = undefined;
    var dims = std.ArrayList(Present).init(allocator);

    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const present_dim = try parse_line(allocator, line);
        try dims.append(present_dim);
    }

    const dims_slice = dims.toOwnedSlice();
    defer allocator.free(dims_slice);

    const answer_part1: u32 = part1(dims_slice);
    const answer_part2: u32 = part2(dims_slice);

    std.debug.print("Day 2\n", .{});
    std.debug.print("  Part 1: {}\n", .{answer_part1});
    std.debug.print("  Part 2: {}\n\n", .{answer_part2});
}

fn part1(dimensions: []const Present) u32 {
    var total: u32 = 0;
    for (dimensions) |dim| {
        total += dim.wrapping_paper();
    }
    return total;
}

test "part1" {
    var presents = [1]Present{.{ .length = 2, .width = 3, .height = 4 }};
    try testing.expect(part1(presents[0..]) == 58);

    presents = [1]Present{.{ .length = 1, .width = 1, .height = 10 }};
    try testing.expect(part1(presents[0..]) == 43);
}

fn part2(dimensions: []const Present) u32 {
    var total: u32 = 0;
    for (dimensions) |dim| {
        total += dim.ribbon();
    }
    return total;
}

test "part2" {
    var presents = [1]Present{.{ .length = 2, .width = 3, .height = 4 }};
    try testing.expect(part2(presents[0..]) == 34);

    presents = [1]Present{.{ .length = 1, .width = 1, .height = 10 }};
    try testing.expect(part2(presents[0..]) == 14);
}

const parse_present = mecha.map(Present, mecha.toStruct(Present), mecha.combine(.{
    mecha.int(u32, .{}),
    mecha.ascii.char('x'),
    mecha.int(u32, .{}),
    mecha.ascii.char('x'),
    mecha.int(u32, .{}),
}));

fn parse_line(allocator: mem.Allocator, line: []const u8) anyerror!Present {
    const pres = try parse_present(allocator, line);
    return pres.value;
}

const Present = struct {
    length: u32,
    width: u32,
    height: u32,

    fn wrapping_paper(self: *const Present) u32 {
        const first_side = self.length * self.width;
        const second_side = self.width * self.height;
        const third_side = self.height * self.length;

        const extra = min(first_side, second_side, third_side);
        return (2 * first_side) + (2 * second_side) + (2 * third_side) + extra;
    }

    fn ribbon(self: *const Present) u32 {
        const first_perim = (2 * self.length) + (2 * self.width);
        const second_perim = (2 * self.width) + (2 * self.height);
        const third_perim = (2 * self.height) + (2 * self.length);

        const volume = self.length * self.width * self.height;

        return min(first_perim, second_perim, third_perim) + volume;
    }
};

fn min(a: u32, b: u32, c: u32) u32 {
    const i = if (a < b) a else b;
    return if (i < c) i else c;
}
