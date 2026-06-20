package core

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
}

type Log struct {
	Status             string   `yaml:"status"`
	Rating             *int     `yaml:"rating"`
	PlayedPlatform     *string  `yaml:"played_platform"`
	Started            *string  `yaml:"started"`
	Finished           *string  `yaml:"finished"`
	AchievementPercent *float32 `yaml:"achievement_percent"`
	HoursPlayed        *float32 `yaml:"hours_played"`
	Revisit            bool     `yaml:"revisit"`
}

type ProviderIds struct {
	Steam *int `yaml:"steam"`
	Igdb  *int `yaml:"igdb"`
}
