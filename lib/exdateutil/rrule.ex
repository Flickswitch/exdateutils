defmodule ExDateUtil.Rrule do
  @moduledoc """
  Rrule parsing and utility functions using precompiled Rust NIF from https://github.com/fmeringdal/rust-rrule

  # Note:
  - All RRules must contain a DTSTART value, e.g. `DTSTART;TZID=Europe/London:20230326T000000Z`
  - All dates must be passed in RFC3339 format, e.g. `"2023-06-19T12:00:01+01:00"`

  # Errors:
  Every function raises `ErlangError` when it cannot parse its input or calculate the occurrences.
  The error's `original` field holds the reason as a string.
  """

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  targets = ~w(
    aarch64-apple-darwin
    aarch64-unknown-linux-gnu
    aarch64-unknown-linux-musl
    x86_64-apple-darwin
    x86_64-unknown-linux-gnu
    x86_64-unknown-linux-musl
  )

  use RustlerPrecompiled,
    otp_app: :exdateutil,
    crate: "exdateutil_rrule",
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("RRULE_BUILD") in ["1", "true"],
    version: version,
    targets: targets

  @doc ~S"""
  Takes an RRule string and a limit integer. Returns a list of strings
  that represent the next `limit` number of occurrences.

  ## Arguments:

  * `rrule`: The RRule string to parse.
  * `limit`: The number of recurrences to return.

  ## Returns:

  A list of strings.

  ## Example

      iex> ExDateUtil.Rrule.next("DTSTART;TZID=Europe/London:20230326T000000Z\nRRULE:FREQ=DAILY;BYDAY=Mo,Tu,We", 2)
      ["2023-03-27T00:00:00.000+01:00", "2023-03-28T00:00:00.000+01:00"]
  """
  def next(_rrule, _limit), do: error()

  @doc ~S"""
  Takes an RRule string, a start date, an end date and a boolean indicating whether to include the
  start date in the results. Returns a list of strings representing the occurrences that fall between
  the start and end dates.

  ## Arguments:

  * `rrule`: The RRule string to parse.
  * `start_date`: The start date of the range to check for occurrences.
  * `end_date`: The end date of the occurrences.
  * `inc`: Whether to include the start date in the results.

  ## Returns:

  A list of strings

  ## Example

      iex> ExDateUtil.Rrule.between("DTSTART;TZID=Europe/London:20230326T000000Z\nRRULE:FREQ=DAILY;BYDAY=Mo,Tu,We", "2023-03-26T00:00:00.000+01:00", "2023-03-29T00:00:00.000+01:00", false)
      ["2023-03-27T00:00:00.000+01:00", "2023-03-28T00:00:00.000+01:00"]

      iex> ExDateUtil.Rrule.between("DTSTART;TZID=Europe/London:20230326T000000Z\nRRULE:FREQ=DAILY;BYDAY=Mo,Tu,We", "2023-03-26T00:00:00.000+01:00", "2023-03-29T00:00:00.000+01:00", true)
      ["2023-03-27T00:00:00.000+01:00", "2023-03-28T00:00:00.000+01:00", "2023-03-29T00:00:00.000+01:00"]
  """
  def between(_rrule, _start_date, _end_date, _inc), do: error()

  @doc ~S"""
  Takes an RRule string, a date string, and a boolean indicating whether to include the
  before date in the results. Returns a list of date strings representing the occurrence
  that happens just before the given date.

  ## Arguments:

  * `rrule`: The RRule string to parse.
  * `before`: The date to find the occurrence before.
  * `inc`: If true, the before date will be included in the results.

  ## Returns:

  A list of strings.

  ## Example

      iex> ExDateUtil.Rrule.just_before("DTSTART;TZID=Europe/London:20230326T000000Z\nRRULE:FREQ=DAILY;BYDAY=Mo,Tu,We", "2023-03-29T00:00:00.000+01:00", false)
      ["2023-03-28T00:00:00.000+01:00"]

      iex> ExDateUtil.Rrule.just_before("DTSTART;TZID=Europe/London:20230326T000000Z\nRRULE:FREQ=DAILY;BYDAY=Mo,Tu,We", "2023-03-29T00:00:00.000+01:00", true)
      ["2023-03-29T00:00:00.000+01:00"]
  """
  def just_before(_rrule, _before_date, _inc), do: error()

  @doc ~S"""
  Takes an RRule string, an after_date string, and a boolean indicating whether to include the
  start date in the results. Returns a list of date strings representing the occurrence that
  happens just after the given date.

  ## Arguments:

  * `rrule`: The RRule string to parse.
  * `after_date`: The date to start looking for occurrences after.
  * `inc`: If true, include the start date in the results.

  ## Returns:

  A list of strings.

  ## Example

      iex> ExDateUtil.Rrule.just_after("DTSTART;TZID=Europe/London:20230326T000000Z\nRRULE:FREQ=DAILY;BYDAY=Mo,Tu,We", "2023-03-29T00:00:00.000+01:00", false)
      ["2023-04-03T00:00:00.000+01:00"]

      iex> ExDateUtil.Rrule.just_after("DTSTART;TZID=Europe/London:20230326T000000Z\nRRULE:FREQ=DAILY;BYDAY=Mo,Tu,We", "2023-03-29T00:00:00.000+01:00", true)
      ["2023-03-29T00:00:00.000+01:00"]
  """
  def just_after(_rrule, _after_date, _inc), do: error()

  @doc ~S"""
  Takes an RRule string and returns a map of the properties.

  ## Arguments:

  * `rrule`: The RRule string to parse.

  ## Returns:

  A map of the RRule's properties.

  ## Example

      iex> ExDateUtil.Rrule.properties("DTSTART;TZID=Europe/London:20230326T000000Z\nRRULE:FREQ=DAILY;BYDAY=Mo,Tu,We")
      %{
        count: "None",
        until: "None",
        __struct__: Properties,
        interval: 1,
        freq: "Daily",
        week_start: "Mon",
        by_set_pos: "[]",
        by_month: [],
        by_month_day: [],
        by_n_month_day: "[]",
        by_year_day: [],
        by_week_no: [],
        by_weekday: ["Every(Mon)", "Every(Tue)", "Every(Wed)"],
        by_hour: [0],
        by_minute: [0],
        by_second: [0],
        by_easter: "None"
      }
  """
  def properties(_rrule), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
