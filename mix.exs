defmodule ExDateUtil.MixProject do
  use Mix.Project

  @source_url "https://github.com/Flickswitch/exdateutils"
  @version "1.1.1"

  def project do
    [
      app: :exdateutil,
      version: @version,
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      aliases: aliases(),
      package: package()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false},
      {:rustler_precompiled, "~> 0.8.2"},
      {:rustler, "~> 0.38.0", optional: true}
    ]
  end

  defp package do
    [
      description: "Rrule parser",
      maintainers: ["Flickswitch engineering"],
      licenses: ["MIT"],
      files: ~w(lib native .formatter.exs README* LICENSE* mix.exs checksum-*.exs),
      links: %{"GitHub" => @source_url}
    ]
  end

  defp aliases do
    [
      fmt: [
        "format",
        "cmd cargo fmt --manifest-path native/exdateutil_rrule/Cargo.toml"
      ]
    ]
  end
end
