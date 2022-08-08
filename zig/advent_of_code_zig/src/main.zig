const std = @import("std");

const y2015 = @import("y2015/solutions.zig");

pub fn main() anyerror!void {
    std.debug.print("Year 2015\n===============\n", .{});
    try y2015.day1.run("data/2015/day1.txt");
    try y2015.day2.run("data/2015/day2.txt");
    try y2015.day3.run("data/2015/day3.txt");
    try y2015.day4.run("data/2015/day4.txt");
}

test {
    _ = y2015;
}
