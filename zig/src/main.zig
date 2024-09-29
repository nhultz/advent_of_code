const std = @import("std");

const y2023 = @import("y2023/solutions.zig");

pub fn main() !void {
    std.debug.print("Year 2023\n===============\n", .{});
    try y2023.run_all();
}
