const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("input.txt");

    try stdout.print("p1 = {}\n", .{p1(input)});
    try stdout.print("p2 = {}\n", .{p2(input)});
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

const number_strings = std.ComptimeStringMap(u32, .{
    .{ "one", '1' },
    .{ "two", '2' },
    .{ "three", '3' },
    .{ "four", '4' },
    .{ "five", '5' },
    .{ "six", '6' },
    .{ "seven", '7' },
    .{ "eight", '8' },
    .{ "nine", '9' },
});

fn p2(input: []const u8) u32 {
    var total: u32 = 0;
    var lines = std.mem.tokenizeAny(u8, input, "\n");

    while (lines.next()) |line| {
        const first_digit: u8 = blk: for (0..line.len) |i| {
            if (std.ascii.isDigit(line[i])) break line[i];

            for (number_strings.kvs) |number| {
                if (line.len - i - 1 < number.key.len) continue;

                if (std.mem.eql(u8, line[i .. i + number.key.len], number.key)) {
                    break :blk @intCast(number.value);
                }
            }
        } else unreachable;

        var i: usize = line.len;
        const second_digit: u8 = blk: while (i >= 0) {
            i -= 1;
            if (std.ascii.isDigit(line[i])) break line[i];

            for (number_strings.kvs) |number| {
                if (line.len - i < number.key.len) continue;

                if (std.mem.eql(u8, line[i .. i + number.key.len], number.key)) {
                    break :blk @intCast(number.value);
                }
            }
        } else unreachable;

        const number = std.fmt.parseUnsigned(u32, &[_]u8{ first_digit, second_digit }, 10) catch unreachable;

        total += number;
    }

    return total;
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
        \\two1nine
        \\eightwothree
        \\abcone2threexyz
        \\xtwone3four
        \\4nineeightseven2
        \\zoneight234
        \\7pqrstsixteen
    ;

    try expectEqual(p2(input), 281);
}
