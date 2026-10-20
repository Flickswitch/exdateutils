defmodule ExDateUtil.RruleTest do
  use ExUnit.Case
  doctest ExDateUtil.Rrule

  test "raises the calculation error as an ErlangError" do
    rrule = "DTSTART;TZID=Etc/UTC:20261019T000000Z\nRRULE:FREQ=MONTHLY;INTERVAL=2000;BYMONTHDAY=1"

    error =
      assert_raise ErlangError, fn ->
        ExDateUtil.Rrule.just_after(rrule, "2026-10-19T00:00:00Z", true)
      end

    assert error.original =~ "2000"
  end

  test "raises the parse error as an ErlangError" do
    error = assert_raise ErlangError, fn -> ExDateUtil.Rrule.properties("RRULE:FREQ=MONTHLY") end

    assert is_binary(error.original)
  end
end
