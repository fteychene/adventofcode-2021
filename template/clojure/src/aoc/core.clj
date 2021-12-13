(ns aoc.core
  (:gen-class)
  (:require [clojure.java.io :as io])
  (:require [aoc.initial :as initial])
  (:require [aoc.updated :as updated]))


(def inputs (slurp (.getFile (io/resource "inputs"))))

(defn -main
  "Application entry point"
  [& args]
  (println "Initial : " (initial/run inputs))
  (println "Updated : " (updated/run inputs)))
