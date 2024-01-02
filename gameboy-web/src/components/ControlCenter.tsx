import {ToggleButton, ToggleButtonGroup} from "@mui/material";
import {BugReport, PlayArrow, SkipNext} from "@mui/icons-material";

export interface ControlCenterProps {
  onPlay: () => void
}

const ControlCenter = (props: ControlCenterProps) => {
  return (
      <>
        <ToggleButtonGroup
        >
          <ToggleButton value="play" onClick={() => props.onPlay()}>
            <PlayArrow/>
          </ToggleButton>
          <ToggleButton value="debug">
            <BugReport/>
          </ToggleButton>
          <ToggleButton value="next" disabled>
            <SkipNext/>
          </ToggleButton>
        </ToggleButtonGroup>
      </>
  )
}

export default ControlCenter