import { compat, matches, types as T } from "../deps.ts";

export const migration: T.ExpectedExports.migration = compat.migrations
  .fromMapping(
    {
      // 0.3.2.1: initial version
      "0.3.2.2": {
        up: compat.migrations.updateConfig(
          (config) => {
            if (
              matches.shape({
                bitcoind: matches.shape({ type: matches.any }),
              }).test(config)
            ) {
              delete config.bitcoind.type;
            }
            return config;
          },
          false,
          { version: "0.3.2.2", type: "up" },
        ),
        down: compat.migrations.updateConfig(
          (config) => {
            if (
              matches.shape({
                bitcoind: matches.shape({ type: matches.any }, ["type"]),
              }).test(config)
            ) {
              config.bitcoind.type = "internal";
            }
            if (
              matches.shape({
                advanced: matches.shape({ "log-level": matches.any }, [
                  "log-level",
                ]),
              }).test(config)
            ) {
              delete config.advanced["log-level"];
            }
            return config;
          },
          false,
          { version: "0.3.2.2", type: "down" },
        ),
      },
      "0.3.2.3": {
        up: compat.migrations.updateConfig(
          (config) => {
            delete config.bitcoind;
            return config;
          },
          false,
          { version: "0.3.2.3", type: "up" },
        ),
        down: compat.migrations.updateConfig(
          (config) => {
            config.bitcoind = {};
            delete config["bitcoind-user"];
            delete config["bitcoind-password"];
            return config;
          },
          false,
          { version: "0.3.2.3", type: "down" },
        ),
      },
      // 0.3.2.4: no migration needed
      // 0.3.2.5: no migration needed
    },
    "0.3.2.5",
  );
