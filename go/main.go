package main

import "github.com/gin-gonic/gin"

func main() {
	g := gin.New()
	g.GET("/", func(c *gin.Context) {
		c.JSON(200, "OK")
	})

	print("Running server on port 3001\n")
	g.Run(":3001")
}
