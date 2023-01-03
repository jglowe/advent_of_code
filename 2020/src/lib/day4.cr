################################################################################
#                               _             _  _
#                              | |           | || |
#                            __| | __ _ _   _| || |_
#                           / _` |/ _` | | | |__   _|
#                          | (_| | (_| | |_| |  | |
#                           \__,_|\__,_|\__, |  |_|
#                                        __/ |
#                                       |___/
#
# Jonathan Lowe
# github : https://github.com/jglowe
#
# The file for day4 advent of code 2020
################################################################################

class Passport
  property byr : String | Nil # (Birth Year)
  property iyr : String | Nil # (Issue Year)
  property eyr : String | Nil # (Expiration Year)
  property hgt : String | Nil # (Height)
  property hcl : String | Nil # (Hair Color)
  property ecl : String | Nil # (Eye Color)
  property pid : String | Nil # (Passport ID)
  property cid : String | Nil # (Country ID)

  def has_required_fields? : Bool
    return !@byr.nil? && !@iyr.nil? && !@eyr.nil? && !@hgt.nil? && !@hcl.nil? \
        && !@ecl.nil? && !@pid.nil?
  end

  def is_valid? : Bool
    if byr = @byr
      if byr.size != 4
        return false
      end

      byr = Int64.new byr

      if byr < 1920 || byr > 2002
        return false
      end
    else
      return false
    end


    if iyr = @iyr
      if iyr.size != 4
        return false
      end

      iyr = Int64.new iyr

      if iyr < 2010 || iyr > 2020
        return false
      end
    else
      return false
    end

    if eyr = @eyr
      if eyr.size != 4
        return false
      end

      eyr = Int64.new eyr

      if eyr < 2020 || eyr > 2030
        return false
      end
    else
      return false
    end

    if hgt = @hgt
      if /^(\d+)cm$/.match hgt
        hgt = Int64.new $1
        if hgt < 150 || hgt > 193
          return false
        end
      elsif /^(\d+)in$/.match hgt
        hgt = Int64.new $1
        if hgt < 59 || hgt > 76
          return false
        end
      else
        return false
      end
    else
      return false
    end

    if hcl = @hcl
      if !/^#[a-f0-9]{6}$/.match hcl
        return false
      end
    else
      return false
    end

    if ecl = @ecl
      if !ecl.in? ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        return false
      end
    else
      return false
    end

    if pid = @pid
      if !/^(\d){9}$/.match pid
        return false
      end
    else
      return false
    end

    return true
  end
end

def parse_passports(input : String) : Array(Passport)
  passport = Passport.new
  passports = Array(Passport).new
  input.each_line do |line|
    if line == ""
      passports.push passport
      passport = Passport.new
    else
      line.split(" ").each do |field|
        parts = field.split(":")
        if parts.size != 2
          raise "Invalid input"
        end

        case parts[0]
        when "byr"
          passport.byr = parts[1]
        when "iyr"
          passport.iyr = parts[1]
        when "eyr"
          passport.eyr = parts[1]
        when "hgt"
          passport.hgt = parts[1]
        when "hcl"
          passport.hcl = parts[1]
        when "ecl"
          passport.ecl = parts[1]
        when "pid"
          passport.pid = parts[1]
        when "cid"
          passport.cid = parts[1]
        else
          raise "Invalid passport field"
        end
      end
    end
  end

  passports.push passport

  return passports
end

def part1(input : String) : Int64
  passports = parse_passports input

  counts = passports.map do |passport|
    if passport.has_required_fields?
      1
    else
      0
    end
  end

  return Int64.new counts.sum
end

def part2(input : String) : Int64
  passports = parse_passports input

  counts = passports.map do |passport|
    if passport.is_valid?
      1
    else
      0
    end
  end

  return Int64.new counts.sum
end
