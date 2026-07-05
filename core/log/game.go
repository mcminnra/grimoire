package log

type Game struct {
	Title       string      `yaml:"title"`
	Description *string     `yaml:"description"`
	ReleaseDate *string     `yaml:"release_date"`
	Developer   *string     `yaml:"developer"`
	Publisher   *string     `yaml:"publisher"`
	Tags        *[]string   `yaml:"tags"`
	Cover       *string     `yaml:"cover"`
	Log         Log         `yaml:"log"`
	ProviderIds ProviderIds `yaml:"provider_ids"`
	Review      string      `yaml:"-"` // This is the "rest" of the markdown
}

type Log struct {
	Status             Status   `yaml:"status"`
	Rating             *int     `yaml:"rating"`
	PlayedPlatform     *string  `yaml:"played_platform"`
	Started            *string  `yaml:"started"`
	Finished           *string  `yaml:"finished"`
	AchievementPercent *float32 `yaml:"achievement_percent"`
	HoursPlayed        *float32 `yaml:"hours_played"`
	Revisit            bool     `yaml:"revisit"`
}

type Status string

const (
	StatusBacklog       Status = "backlog"
	StatusPlaying       Status = "playing"
	StatusPlayed        Status = "played"
	StatusAbandoned     Status = "abandoned"
	StatusCompleted     Status = "completed"
	StatusCompletedPlus Status = "completed+"
	StatusMastered      Status = "mastered"
)

func (s Status) Valid() bool {
	switch s {
	case StatusBacklog, StatusPlaying, StatusPlayed, StatusAbandoned, StatusCompleted, StatusCompletedPlus, StatusMastered:
		return true
	}
	return false
}

type ProviderIds struct {
	Steam *int `yaml:"steam"`
	Igdb  *int `yaml:"igdb"`
}
