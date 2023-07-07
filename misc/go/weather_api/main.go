package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"sort"
	"time"

	"github.com/joho/godotenv"
)

type Response struct {
	List []struct {
		DateTimeText string                        `json:"dt_txt"`
		Main         ForecastOutputTemperatureList `json:"main"`
	} `json:"list"`
	City struct {
		Name string `json:"name"`
	} `json:"city"`
}

type ForecastOutputTemperatureList struct {
	Temp float32 `json:"temp"`
}

type ForecastOutput struct {
	DayDate  int
	DateTime string
	TempList []ForecastOutputTemperatureList
}

func main() {

	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	resp, err := http.Get(fmt.Sprintf("https://api.openweathermap.org/data/2.5/forecast?lat=-6.2182&lon=106.858398&appid=%s&units=metric", os.Getenv("OPENWEATHER_KEY")))
	if err != nil {
		fmt.Println("err: ", err)
	}

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)

	var result Response
	err = json.Unmarshal(body, &result)
	if err != nil {
		fmt.Println("err", err)
	}

	var outputlist = make(map[int]ForecastOutput)

	for _, date := range result.List {
		dateTimeText, err := time.Parse("2006-01-02 15:04:05", date.DateTimeText)
		if err != nil {
			panic(err)
		}
		parseDate := fmt.Sprintf("%s, %d %s %d",
			dateTimeText.Weekday().String(),
			dateTimeText.Day(),
			dateTimeText.Month(),
			dateTimeText.Year(),
		)

		outputlist[dateTimeText.Day()] = ForecastOutput{
			dateTimeText.Day(),
			parseDate,
			append(outputlist[dateTimeText.Day()].TempList, date.Main),
		}

	}

	// Process the response to terminal output
	var outputs []ForecastOutput

	for _, outp := range outputlist {
		outputs = append(outputs, outp)
	}

	sort.Slice(outputs, func(i, j int) bool {
		return outputs[i].DayDate < outputs[j].DayDate
	})

	fmt.Println("Weather Forecast:")
	for _, outs := range outputs {
		fmt.Printf("%s: %.2f °C\n", outs.DateTime, outs.TempList[0].Temp)
	}
}
