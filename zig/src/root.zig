const std = @import("std");

const solutions_2023 = @import("y2023/solutions.zig");

pub fn download_input(year: u16, day: ?u8) void {
    if (day) |d| {
        std.log.info("Downloading input file for {d} day {d}", .{ year, d });
    } else {
        std.log.info("Downloading all input files for {d}", .{year});
    }
}

pub fn run_solutions(year: u16, day: ?u8) !void {
    if (day) |d| {
        std.log.info("Running solutions for {d} day {d}", .{ year, d });
    } else {
        std.log.info("Running all solutions for {d}", .{year});
    }

    try solutions_2023.run_all();
}
