module YouShouldBeAbleToSolveThis
  # NOTE 0 == 0 all the time is boring, so we'll do something else.
  class Variable
    def -(other)
      other
    end

    def coerce(other)
      [-12, other]
    end
  end
end

# NOTE Not *good* Ruby, but we'll do it this way to keep the "formula"
# as simple as possible.
def x
  YouShouldBeAbleToSolveThis::Variable.new
end
