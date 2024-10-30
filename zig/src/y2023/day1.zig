const std = @import("std");

const numberConversion = std.StaticStringMap(u32).initComptime(.{
    .{ "0", 0 },
    .{ "1", 1 },
    .{ "2", 2 },
    .{ "3", 3 },
    .{ "4", 4 },
    .{ "5", 5 },
    .{ "6", 6 },
    .{ "7", 7 },
    .{ "8", 8 },
    .{ "9", 9 },
    .{ "zero", 0 },
    .{ "one", 1 },
    .{ "two", 2 },
    .{ "three", 3 },
    .{ "four", 4 },
    .{ "five", 5 },
    .{ "six", 6 },
    .{ "seven", 7 },
    .{ "eight", 8 },
    .{ "nine", 9 },
});

pub fn run(input: []const u8) !void {
    const answer_part1: u64 = try part1(input);
    const answer_part2: u64 = try part2(input);

    std.log.info("Day 1", .{});
    std.log.info("  Part 1: {}", .{answer_part1});
    std.log.info("  Part 2: {}\n", .{answer_part2});
}

fn part1(input: []const u8) !u64 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var sum: u64 = 0;

    while (lines.next()) |line| {
        var i: usize = 0;
        var first_digit: u32 = undefined;
        while (i < line.len) : (i += 1) {
            if (std.ascii.isDigit(line[i])) {
                first_digit = numberConversion.get(line[i .. i + 1]).?;
                break;
            }
        }

        var j: usize = line.len - 1;
        var second_digit: u32 = undefined;
        while (j >= 0) : (j -= 1) {
            if (std.ascii.isDigit(line[j])) {
                second_digit = numberConversion.get(line[j .. j + 1]).?;
                break;
            }
        }

        sum += (first_digit * 10) + second_digit;
    }

    return sum;
}

fn part2(input: []const u8) !u64 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var sum: u64 = 0;

    while (lines.next()) |line| {
        var i: usize = 0;
        var first_digit: u32 = undefined;
        search_loop: while (i < line.len) : (i += 1) {
            for (numberConversion.keys()) |key| {
                if (std.mem.startsWith(u8, line[i..line.len], key)) {
                    first_digit = numberConversion.get(key).?;
                    break :search_loop;
                }
            }
        }

        var j: usize = line.len;
        var second_digit: u32 = undefined;
        search_loop: while (j > 0) : (j -= 1) {
            for (numberConversion.keys()) |key| {
                if (std.mem.endsWith(u8, line[0..j], key)) {
                    second_digit = numberConversion.get(key).?;
                    break :search_loop;
                }
            }
        }

        sum += (first_digit * 10) + second_digit;
    }

    return sum;
}
