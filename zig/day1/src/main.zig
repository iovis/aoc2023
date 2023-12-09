const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    try stdout.print("p1 = {}\n", .{p1(input)});
}

/// Find the first and last digit in a string
/// Combine them
/// Sum all of them
fn p1(input: []const u8) u32 {
    var total: u32 = 0;
    var lines = std.mem.tokenizeAny(u8, input, "\n");

    while (lines.next()) |line| {
        const first_digit = for (0..line.len) |i| {
            if (std.ascii.isDigit(line[i])) break line[i];
        } else unreachable;

        var i: usize = line.len - 1;
        const second_digit = while (i >= 0) : (i -= 1) {
            if (std.ascii.isDigit(line[i])) break line[i];
        } else unreachable;

        const number = std.fmt.parseUnsigned(u32, &[_]u8{ first_digit, second_digit }, 10) catch unreachable;

        total += number;
    }

    return total;
}

fn p2(input: []const u8) u32 {
    _ = input;
    return 32;
}

const expectEqual = std.testing.expectEqual;

test "p1" {
    const input: []const u8 =
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;

    try expectEqual(p1(input), 142);
}

test "p2" {
    const input: []const u8 =
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;

    try expectEqual(p2(input), 281);
}
