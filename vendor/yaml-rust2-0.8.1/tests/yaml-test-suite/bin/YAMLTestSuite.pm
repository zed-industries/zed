#------------------------------------------------------------------------------
# Common code for the bin/suite-to-* programs.
#------------------------------------------------------------------------------

package YAMLTestSuite;
use strict; use warnings;

use utf8;
use autodie qw(open close);
use Encode;
use MIME::Base64;
use YAML::PP;

my $ypp = YAML::PP->new;

sub new {
    my $class = shift;
    my $self = bless {
        data => undef,
        id => undef,
        num => undef,
        ID => undef,
        file => 0,
        skip => 0,
        make => 0,
    }, $class;
    return $self;
}

sub initial {}
sub done {}
sub final {}
sub skip {1}

sub run {
    my ($self, $files) = @_;

    $self->initial;

    for my $file (@$files) {
        $self->{file}++;

        $file =~ m{^.*/(.*)\.yaml$} or die;
        $self->{id} = $1;

        # next unless $1 eq '6BFJ';

        my $data = $ypp->load_file($file);

        if ($data->[0]{skip} and $self->skip) {
            $self->{skip}++;
            next;
        }

        my $multi = $self->{multi} = (@$data > 1) || 0;
        my $l = $multi
            ? int(log(@$data - 1) / log(10)) + 2
            : 2;
        my $cache = {};
        my $i = 0;

        for my $test (@$data) {
            $self->{make}++;
            $self->{data} = $test;
            $self->{num} = sprintf "%0${l}d", $i++;
            my $ID = $self->{ID} = $multi
                ? "$self->{id}-$self->{num}"
                : $self->{id};

            die "Can't change test name in '$ID'"
                if $test->{name} and $cache->{name};

            $test->{name} ||= $cache->{name} or die;
            $test->{tags} ||= $cache->{tags} || '';
            $test->{yaml} ||= $cache->{yaml} or die;
            $test->{fail} = exists $test->{fail} ? 1 : 0;

            $self->{slug} = lc $test->{name};
            $self->{slug} =~ s/[^\w]+/-/g;
            $self->{slug} =~ s/^-//;
            $self->{slug} =~ s/-$//;

            for my $key (qw< tree json dump emit toke >) {
                if (
                    not exists $test->{$key} and
                    defined $cache->{$key}
                ) {
                    $test->{$key} = $cache->{$key};
                }
            }

            $cache = { %$test };

            $self->make;
        }

        $self->done;
    }

    $self->final;
}

sub unescape {
    my ($self, $text) = @_;

    $text =~ s/␣/ /g;
    $text =~ s/—*»/\t/g;
    $text =~ s/←/\r/g;
    $text =~ s/⇔/x{FEFF}/g;
    $text =~ s/↵//g;
    $text =~ s/∎\n\z//;

    return $text;
}

sub play_url {
    my ($self, $text) = @_;

    $text = encode_utf8 $self->unescape($text);

    my $base64 = encode_base64($text);
    $base64 =~ s{\n}{}g;
    $base64 =~ s{\+}{-}g;
    $base64 =~ s{/}{_}g;

    return "https://play.yaml.io/main/parser?input=$base64";
}

1;
