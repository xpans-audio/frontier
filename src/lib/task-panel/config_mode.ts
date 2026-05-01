type HeadphonesConfigMode = {
  mode: "headphones";
  config: HeadphonesConfiguration;
};
type StereoConfigMode = {
  mode: "stereo";
  config: StereoConfiguration;
};
type MonoConfigMode = {
  mode: "mono";
  config: MonoConfiguration;
};

export type ConfigMode =
  | HeadphonesConfigMode
  | StereoConfigMode
  | MonoConfigMode;

export type PanLaw = "square_root" | "natural" | "sine";

export type HeadphonesConfiguration = {
  pan_law: PanLaw;
  max_itd_nanos: number;
  distance_curve: DistanceCurve;
  distance_effect: number;
  min_distance: number;
  max_distance: number;
};

export type DistanceCurve = "linear" | "exponential" | "square_root" | "sine";

export type StereoConfiguration = {
  pan_law: PanLaw;
  mode: StereoMode;
};

export type StereoMode = "directional" | "positional";

export type MonoConfiguration = {
  channels: number;
};

export function new_headphones(): HeadphonesConfigMode {
  let config: HeadphonesConfiguration = {
    pan_law: "sine",
    max_itd_nanos: 500000,
    distance_curve: "exponential",
    distance_effect: 0.0,
    min_distance: 0,
    max_distance: 1,
  };
  return {
    mode: "headphones",
    config,
  };
}

export function new_stereo(): StereoConfigMode {
  let config: StereoConfiguration = {
    pan_law: "sine",
    mode: "positional",
  };
  return {
    mode: "stereo",
    config,
  };
}

export function new_mono(): MonoConfigMode {
  let config: MonoConfiguration = {
    channels: 1,
  };
  return {
    mode: "mono",
    config,
  };
}

export const Modes = {
  headphones: "headphones",
  stereo: "stereo",
  mono: "mono",
};

export const ModeNames = new Map([
  ["headphones", "Headphones"],
  ["stereo", "Stereo"],
  ["mono", "Mono"],
]);

export type Mode = "headphones" | "stereo" | "mono";
export type Config =
  | HeadphonesConfiguration
  | StereoConfiguration
  | MonoConfiguration;

export function new_config_mode(mode: Mode): ConfigMode {
  switch (mode) {
    case "headphones": {
      return new_headphones();
    }
    case "stereo": {
      return new_stereo();
    }
    case "mono": {
      return new_mono();
    }
  }
}
