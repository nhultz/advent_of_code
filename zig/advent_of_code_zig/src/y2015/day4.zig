const std = @import("std");
const fs = std.fs;
const fmt = std.fmt;
const Md5 = std.crypto.hash.Md5;
const mem = std.mem;
const testing = std.testing;

pub fn run(input_file: []const u8) anyerror!void {
    var input = try std.fs.cwd().openFile(input_file, .{});
    defer input.close();

    var buf: [1024 * 1024]u8 = undefined;
    const bytes_read = try input.readAll(&buf);

    const answer_part1: u64 = try part1(buf[0..bytes_read]);
    const answer_part2: u64 = try part2(buf[0..bytes_read]);

    std.debug.print("Day 4\n", .{});
    std.debug.print("  Part 1: {}\n", .{answer_part1});
    std.debug.print("  Part 2: {}\n\n", .{answer_part2});
}

fn part1(input: []const u8) !u64 {
    var static_buf = [_]u8{0} ** 64;

    var n: u64 = 1;
    while (true) : (n += 1) {
        const check = try fmt.bufPrint(&static_buf, "{s}{d}", .{ input, n });

        var out: [Md5.digest_length]u8 = undefined;
        Md5.hash(check, &out, .{});

        if (out[0] == 0 and out[1] == 0 and out[2] <= 15) {
            break;
        }
    }

    return n;
}

test "part1" {
    try testing.expect((try part1("abcdef")) == 609043);
    try testing.expect((try part1("pqrstuv")) == 1048970);
}

fn part2(input: []const u8) !u64 {
    var static_buf = [_]u8{0} ** 64;

    var n: u64 = 1;
    while (true) : (n += 1) {
        const check = try fmt.bufPrint(&static_buf, "{s}{d}", .{ input, n });

        var out: [Md5.digest_length]u8 = undefined;
        Md5.hash(check, &out, .{});

        if (out[0] == 0 and out[1] == 0 and out[2] == 0) {
            break;
        }
    }

    return n;
}

test "part2" {}
