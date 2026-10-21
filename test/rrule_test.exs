defmodule ExDateUtil.RruleTest do
  use ExUnit.Case
  doctest ExDateUtil.Rrule

  test "returns occurrences for a large INTERVAL and none for a rule that never matches" do
    large_interval =
      "DTSTART;TZID=Etc/UTC:20261019T000000Z\nRRULE:FREQ=MONTHLY;INTERVAL=2000;BYMONTHDAY=1"

    assert ExDateUtil.Rrule.just_after(large_interval, "2026-10-19T00:00:00Z", true) ==
             ["2193-06-01T00:00:00.000Z"]

    february_30 =
      "DTSTART;TZID=Etc/UTC:20240101T000000Z\nRRULE:FREQ=YEARLY;BYMONTH=2;BYMONTHDAY=30"

    assert ExDateUtil.Rrule.next(february_30, 3) == []
    assert ExDateUtil.Rrule.just_after(february_30, "2024-01-01T00:00:00Z", true) == []
  end

  test "properties keeps negative BYMONTHDAY values in by_n_month_day" do
    start = "DTSTART;TZID=Etc/UTC:20240101T080000Z\n"

    month_end = ExDateUtil.Rrule.properties(start <> "RRULE:FREQ=MONTHLY;BYMONTHDAY=-1")
    assert %{freq: "Monthly", by_month_day: [], by_n_month_day: "[-1]"} = month_end

    mixed = ExDateUtil.Rrule.properties(start <> "RRULE:FREQ=MONTHLY;BYMONTHDAY=1,15,-1,-2")
    assert %{by_month_day: [1, 15], by_n_month_day: "[-2, -1]", by_hour: [8]} = mixed

    last_weekday =
      ExDateUtil.Rrule.properties(start <> "RRULE:FREQ=MONTHLY;BYDAY=MO,FR;BYSETPOS=-1")

    assert %{by_set_pos: "[-1]", by_weekday: ["Every(Mon)", "Every(Fri)"]} = last_weekday

    nth_weekdays =
      ExDateUtil.Rrule.properties(start <> "RRULE:FREQ=MONTHLY;INTERVAL=2;BYDAY=-1SU,3FR")

    assert %{interval: 2, by_weekday: ["Nth(-1, Sun)", "Nth(3, Fri)"]} = nth_weekdays
  end

  test "next resolves negative BYMONTHDAY, BYSETPOS and Nth weekdays" do
    start = "DTSTART;TZID=Etc/UTC:20240101T080000Z\n"

    assert ExDateUtil.Rrule.next(start <> "RRULE:FREQ=MONTHLY;BYMONTHDAY=-1", 3) ==
             ["2024-01-31T08:00:00.000Z", "2024-02-29T08:00:00.000Z", "2024-03-31T08:00:00.000Z"]

    last_weekday_rule = start <> "RRULE:FREQ=MONTHLY;BYDAY=MO,TU,WE,TH,FR;BYSETPOS=-1"

    assert ExDateUtil.Rrule.next(last_weekday_rule, 3) ==
             ["2024-01-31T08:00:00.000Z", "2024-02-29T08:00:00.000Z", "2024-03-29T08:00:00.000Z"]

    assert ExDateUtil.Rrule.next(start <> "RRULE:FREQ=MONTHLY;BYDAY=3FR", 3) ==
             ["2024-01-19T08:00:00.000Z", "2024-02-16T08:00:00.000Z", "2024-03-15T08:00:00.000Z"]
  end

  test "a DTSTART in UTC iterates in its TZID time zone" do
    rrule = "DTSTART;TZID=Europe/London:20230601T000000Z\nRRULE:FREQ=DAILY"

    assert ExDateUtil.Rrule.next(rrule, 2) ==
             ["2023-06-01T01:00:00.000+01:00", "2023-06-02T01:00:00.000+01:00"]

    assert %{by_hour: [1]} = ExDateUtil.Rrule.properties(rrule)
  end

  test "raises the parse error as an ErlangError" do
    error = assert_raise ErlangError, fn -> ExDateUtil.Rrule.properties("RRULE:FREQ=MONTHLY") end

    assert is_binary(error.original)
  end
end
